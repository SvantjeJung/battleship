extern crate chan;
extern crate rand;

use ::bincode;
use ::bincode::serde::{serialize_into, deserialize_from, DeserializeError};
use ::model;
use ::model::types::{Board, Player, PlayerType, SubField};
use ::net::{self, types};
use ::net::types::{MessageType};
use ::util;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use term_painter::ToStyle;
use term_painter::Color::*;

#[derive(Debug)]
enum CurrentPlayer {
    Host,
    Client,
}

/// Initialize and prepare game
pub fn init(name: String, size: u8, board: Vec<SubField>) {
    let listener = TcpListener::bind((types::LOCALHOST, types::DEFAULT_PORT)).unwrap();

    // accept one incoming connection
    let (client_conn, _) = listener.accept().unwrap();
    let mut client_stream = client_conn.try_clone().unwrap();

    // welcome client
    serialize_into(
        &mut client_stream,
        &(types::MessageType::Welcome(
            "Welcome stranger, let me sink your ships!".to_string(),
            name.clone())
        ),
        bincode::SizeLimit::Infinite
    );

    // add CTRL+C system hook, so that connection partner is informed about disconnect
    let mut client_stream_clone = client_stream.try_clone().unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ::ctrlc::set_handler(move || {
        net::send(&mut client_stream_clone.try_clone().unwrap(), MessageType::Quit);
        client_stream_clone.shutdown(Shutdown::Both).expect("shutdown call failed");
    }).expect("Error setting Ctrl+C handler");

    // wait for client to send his name
    let recv: Result<types::MessageType, DeserializeError> =
        deserialize_from(&mut client_stream, bincode::SizeLimit::Infinite);
    let client_name = match recv {
        Ok(received) => {
            match received {
                MessageType::Login(name) => {
                    name
                },
                MessageType::Quit => {
                    println!("Client closed connection.");
                    return
                },
                _ => {
                    // unexpected packet

                    "".to_string()
                },
            }
        },
        Err(_) => {
            println!("ERROR hit me");
            "".to_string()
        },
    };

    // create players
    let host = Player {
        own_board: board.clone(),
        op_board: Board::init(),
        player_type: PlayerType::Human,
        name: name,
        capacity: Board::targets(&board),
    };

    let client = Player {
        own_board: Board::init(),
        op_board: Board::init(),
        player_type: PlayerType::Human,
        name: client_name,
        capacity: Board::targets(&Board::init()),
    };

    // start game
    start(host, client, client_stream);

    println!("\n{}", Yellow.paint("Bye."));
}

/// Starting the game with given parameters
fn start(mut host: Player, mut client: Player, mut stream: TcpStream) {
    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                    Request initial board configuration from host                          //
    ///////////////////////////////////////////////////////////////////////////////////////////////

    if host.capacity == 0 {
        net::send(
            &mut stream,
            MessageType::Text("Server is setting its ships, please wait :)".to_string())
        );
        println!("Please set your ships:");
        ::model::place_ships(&mut host);
    }
    model::print_boards(&host);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                    Request initial board configuration from client                        //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    net::send(&mut stream, MessageType::RequestBoard);

    loop {
        let recv: Result<types::MessageType, DeserializeError> =
        deserialize_from(&mut stream, bincode::SizeLimit::Infinite);
        match recv {
            Ok(received) => {
                match received {
                    MessageType::Board(vec) => {
                        client.own_board = vec.clone();
                        client.capacity = Board::targets(&vec);
                        break;
                    },
                    MessageType::Quit => {
                        println!("Client closed connection.");
                        return
                    },
                    _ => continue,
                }
            },
            Err(_) => {
                println!("ERROR board");
                net::send(&mut stream, MessageType::Quit);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                return
            },
        };
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                              Choose random start player                                   //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut current_player = match choose_player(&host, &client).name == host.name {
        true => CurrentPlayer::Host,
        false => CurrentPlayer::Client,
    };
    println!("Starting player: {:?}", current_player);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                             Take turns while not ended                                    //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    loop {
        match current_player {
            CurrentPlayer::Host => {
                // inform Client that its the turn of Host
                net::send(&mut stream, MessageType::TurnHost);

                // wait for input from Host
                println!("{}", Yellow.paint("It's your turn!"));
                let mut coord;
                loop {
                    println!("{}", Yellow.paint("Please enter a valid coordinate: "));
                    coord = util::read_string();
                    if ::model::valid_coordinate(&coord) {
                        break;
                    }
                    print!("{}", Red.paint("Invalid coordinate! "));
                }
                // modify boards
                let coord_id = Board::get_index(&coord);
                match ::model::match_move(&mut host, &mut client, coord_id) {
                    SubField::Hit => {
                        net::send(&mut stream, MessageType::Hit(coord_id));
                        model::print_boards(&host);
                    }
                    SubField::Miss => {
                        net::send(&mut stream, MessageType::Miss(coord_id));
                        model::print_boards(&host);
                        current_player = CurrentPlayer::Client;
                    }
                    _ => {}
                }

                // if Host won: send message to Client, end game
                if ::model::game_over(&client) {
                    net::send(&mut stream, MessageType::Lost);
                    println!("{}", Yellow.paint("Congratulations, you won the game :)"));
                    break;
                }
            }
            CurrentPlayer::Client => {
                println!(
                    "{} {} {}",
                    Cyan.paint("Wait for"),
                    Yellow.paint(&client.name),
                    Cyan.paint("to finish turn!"),
                );
                // inform Client that its his turn
                net::send(&mut stream, MessageType::RequestCoord);
                // wait for input from Client
                let recv: Result<types::MessageType, DeserializeError> =
                    deserialize_from(&mut stream, bincode::SizeLimit::Infinite);
                let coordinate = match recv {
                    Ok(received) => {
                        match received {
                            MessageType::Shoot(coord) => {
                                coord
                            },
                            MessageType::Quit => {
                                println!("Client closed connection.");
                                return
                            },
                            _ => {
                                // unexpected packet
                                "".to_string()
                            },
                        }
                    },
                    Err(_) => {
                        println!("ERROR coord");
                        return
                    },
                };

                // modify boards
                let coord_id = Board::get_index(&coordinate);
                match ::model::match_move(&mut client, &mut host, coord_id) {
                    SubField::Hit => {
                        println!("{} hit one of your ships!", client.name);
                        net::send(&mut stream, MessageType::Hit(coord_id));
                        model::print_boards(&host);
                    }
                    SubField::Miss => {
                        println!("{} missed your ships.", client.name);
                        net::send(&mut stream, MessageType::Miss(coord_id));
                        model::print_boards(&host);
                        current_player = CurrentPlayer::Host;
                    }
                    _ => {}
                }

                // if Client won: send message to Client, end game
                if ::model::game_over(&host) {
                    net::send(&mut stream, MessageType::Won);
                    println!("{}", Yellow.paint("You lost :("));
                    break;
                }
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                                  Quit game                                                //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    net::send(&mut stream, MessageType::Quit);
}

/// Randomly choose host or client player to start guessing
fn choose_player<'a>(p1: &'a Player, p2: &'a Player) -> &'a Player {
    use self::rand::distributions::{IndependentSample, Range};
    let between = Range::new(0, 2);
    let mut rng = rand::thread_rng();
    match between.ind_sample(&mut rng) {
        0 => &p1,
        _ => &p2,
    }
}
