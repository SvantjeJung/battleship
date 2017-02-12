pub mod net;
use ::bincode::serde::{serialize_into, deserialize_from, DeserializeError};
use ::bincode;
use ::model::{helper};
use ::model::types::{Board, Player, SubField};
use self::net::types;
use self::net::types::{MessageType};
use std::net::{TcpListener, TcpStream};
//use std::thread;
use term_painter::ToStyle;
use term_painter::Color::*;

extern crate chan;
extern crate rand;

#[derive(Debug)]
enum CurrentPlayer {
    Host,
    Client,
}

pub fn init(name: String, size: u8) {
    let listener = TcpListener::bind((types::LOCALHOST, types::DEFAULT_PORT)).unwrap();

    // accept one incoming connection
    let (client_conn, _) = listener.accept().unwrap();
    let mut client_stream = client_conn.try_clone().unwrap();

    // welcome client
    serialize_into(
        &mut client_stream,
        &(types::MessageType::Welcome("Willkommen".to_string(), name.clone())),
        bincode::SizeLimit::Infinite
    );

    // wait for client to send his name
    let recv: Result<types::MessageType, DeserializeError> =
        deserialize_from(&mut client_stream, bincode::SizeLimit::Infinite);
    let client_name = match recv {
        Ok(received) => {
            println!("RP: {:?}", received);
            match received {
                MessageType::Login(name) => {
                    name
                },
                _ => {
                    // unexpected packet

                    "".to_string()
                },
            }
        },
        Err(_) => {
            println!("ERROR");
            "".to_string()
        },
    };

    // create players
    let host = Player {
        own_board: vec![SubField::Water; 100],
        op_board: vec![SubField::Water; 100],
        player_type: ::model::types::PlayerType::Human,
        name: name,
        capacity: 30,
    };

    let client = Player {
        own_board: vec![SubField::Water; 100],
        op_board: vec![SubField::Water; 100],
        player_type: ::model::types::PlayerType::Human,
        name: client_name,
        capacity: 30,
    };

    // start game
    start(host, client, client_stream);

    println!("Bye.");
}

fn start(mut host: Player, mut client: Player, mut stream: TcpStream) {
    // for testing purpose
    const W: SubField = SubField::Water;
    const S: SubField = SubField::Ship;
    let testboard = vec![
        W,W,S,S,S,S,S,W,W,S,
        W,W,W,W,W,W,W,W,W,S,
        W,S,W,S,S,S,W,W,W,S,
        W,S,W,W,W,W,W,W,W,W,
        W,S,W,S,W,S,S,W,W,S,
        W,S,W,S,W,W,W,W,W,S,
        W,W,W,W,W,W,W,W,W,W,
        W,W,W,W,W,W,S,S,S,S,
        W,S,S,S,W,W,W,W,W,W,
        W,W,W,W,W,W,S,S,W,W,
    ];

    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///                    Request initial board configuration from host                        ///
    ///////////////////////////////////////////////////////////////////////////////////////////////
    println!("Please set your ships: ");
    // TODO modify model::place()
    println!("{}", Red.paint("TODO implement!"));
    host.own_board = testboard;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///                    Request initial board configuration from client                      ///
    ///////////////////////////////////////////////////////////////////////////////////////////////
    net::send(&mut stream, MessageType::RequestBoard);

    loop {
        let recv: Result<types::MessageType, DeserializeError> =
        deserialize_from(&mut stream, bincode::SizeLimit::Infinite);
        match recv {
            Ok(received) => {
                println!("RP: {:?}", received);
                match received {
                    MessageType::Board(vec) => {
                        client.own_board = vec;
                        break;
                    },
                    _ => continue,
                }
            },
            Err(_) => {
                println!("ERROR");
                "".to_string();
                continue
            },
        };
    }

    net::send(&mut stream, MessageType::Board(client.own_board.clone()));

    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///                             Choose random start player                                  ///
    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut current_player = match choose_player(&host, &client).name == host.name {
        true => CurrentPlayer::Host,
        false => CurrentPlayer::Client,
    };
    println!("Starting player: {:?}", current_player);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///                             Take turns while not ended                                  ///
    ///////////////////////////////////////////////////////////////////////////////////////////////
    loop {
        match current_player {
            CurrentPlayer::Host => {
                // inform Client that its the turn of Host
                net::send(&mut stream, MessageType::TurnHost);

                // wait for input from Host
                println!("It's your turn!");
                let mut coord;
                loop {
                    println!("Please enter a valid coordinate: ");
                    coord = ::helper::read_string();
                    if ::model::valid_coordinate(&coord) {
                        break;
                    }
                    print!("{}", Red.paint("Invalid coordinate! "));
                }
                // modify boards
                let coord_id = Board::get_index(&coord);
                match ::model::match_move(&mut host, &mut client, coord_id) {
                    SubField::Hit => {
                        println!("Hit a ship!");
                        net::send(&mut stream, MessageType::Hit(coord_id));
                        ::model::print_boards(&host.own_board, &host.op_board);
                    }
                    SubField::Miss => {
                        println!("Unfortunately a miss.");
                        net::send(&mut stream, MessageType::Miss(coord_id));
                        ::model::print_boards(&host.own_board, &host.op_board);
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
                // inform Client that its his turn
                net::send(&mut stream, MessageType::RequestCoord);
                // wait for input from Client
                let recv: Result<types::MessageType, DeserializeError> =
                    deserialize_from(&mut stream, bincode::SizeLimit::Infinite);
                let coordinate = match recv {
                    Ok(received) => {
                        println!("RP: {:?}", received);
                        match received {
                            MessageType::Shoot(coord) => {
                                coord
                            },
                            _ => {
                                // unexpected packet
                                "".to_string()
                            },
                        }
                    },
                    Err(_) => {
                        println!("ERROR");
                        "".to_string()
                    },
                };

                // modify boards
                let coord_id = Board::get_index(&coordinate);
                match ::model::match_move(&mut client, &mut host, coord_id) {
                    SubField::Hit => {
                        println!("{} hit one of your ships!", client.name);
                        net::send(&mut stream, MessageType::Hit(coord_id));
                        ::model::print_boards(&host.own_board, &host.op_board);
                    }
                    SubField::Miss => {
                        println!("{} missed your ships.", client.name);
                        net::send(&mut stream, MessageType::Miss(coord_id));
                        ::model::print_boards(&host.own_board, &host.op_board);
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
    ///                                 Quit game                                               ///
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
