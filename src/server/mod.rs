extern crate chan;
extern crate rand;

use bincode;
use bincode::serde::deserialize_from;
use model;
use model::types::{Board, Player, PlayerType, SubField};
use net::{self, types};
use net::types::{MessageType};
use util;
use std::net::{Shutdown, TcpListener, TcpStream};
use term_painter::ToStyle;
use term_painter::Color::*;

#[derive(Debug)]
enum CurrentPlayer {
    Host,
    Client,
}

pub struct Server {
    pub ip: &'static str,
    pub port: u16,
    pub host_name: String,
    pub host_board: [[SubField; 10]; 10],
    pub board_dim: u8,
}

/// Initialize and prepare game
pub fn init(server: Server) {
    let listener = TcpListener::bind((server.ip, server.port)).unwrap();

    // accept one incoming connection
    let (client_conn, _) = listener.accept().unwrap();
    let mut client_stream = client_conn.try_clone().unwrap();

    // add CTRL+C system hook, so that connection partner is informed about disconnect
    let client_stream_clone = client_stream.try_clone().unwrap();
    ::ctrlc::set_handler(move || {
        net::send(&mut client_stream_clone.try_clone().unwrap(), MessageType::Quit);
        client_stream_clone.shutdown(Shutdown::Both).expect("shutdown call failed");
    }).expect("Error setting Ctrl+C handler");

    // welcome client
    net::send(
        &mut client_stream,
        MessageType::Welcome(
            "Welcome stranger, let me sink your ships!".to_string(),
            server.host_name.clone())
    );

    // wait for client to send his name
    let recv: Result<types::MessageType, _> =
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
                    net::send(&mut client_stream, MessageType::Unexpected);
                    net::send(&mut client_stream, MessageType::Quit);
                    panic!("Received unexpected packet!")
                },
            }
        },
        Err(_) => {
            Red.with(|| println!("ERROR while waiting for Login"));
            net::send(&mut client_stream, MessageType::Quit);
            client_stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            return;
        },
    };

    // create players
    let host = Player {
        own_board: server.host_board.clone(),
        op_board: Board::init(),
        player_type: PlayerType::Human,
        name: server.host_name,
        capacity: Board::targets(&server.host_board),
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

    Yellow.with(|| println!("\nBye."));
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
        loop {
            match model::place_ships(&mut host) {
                Ok(()) => { break; },
                Err(model::types::ErrorType::DeadEndHuman) => {
                    Red.with(|| println!(
                        "No suitable position left, please restart the ship placement.")
                    );
                    model::restart_placement(&mut host);
                },
                Err(_) => {
                    Red.with(|| println!("Failed placing ships!"));
                    net::send(&mut stream, MessageType::Quit);
                    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                    return
                },
            }
        }
    }
    model::print_boards(&host);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                    Request initial board configuration from client                        //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    net::send(&mut stream, MessageType::RequestBoard);

    loop {
        let recv: Result<types::MessageType, _> =
            deserialize_from(&mut stream, bincode::SizeLimit::Infinite);
        match recv {
            Ok(received) => {
                match received {
                    MessageType::Board(vec) => {
                        client.set_board(vec);
                        break;
                    },
                    MessageType::Quit => {
                        println!("Client closed connection.");
                        return
                    },
                    _ => {
                        Red.with(|| println!("Unexpected packet!"));
                        net::send(&mut stream, MessageType::Quit);
                        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                        return
                    },
                }
            },
            Err(_) => {
                Red.with(|| println!("ERROR board"));
                net::send(&mut stream, MessageType::Quit);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                return
            },
        };
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                              Choose random start player                                   //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut current_player = if rand::random() {
        CurrentPlayer::Host
    } else {
        CurrentPlayer::Client
    };
    // println!("Starting player: {:?}", Yellow.paint(&current_player));

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //                             Take turns while not ended                                    //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    loop {
        match current_player {
            CurrentPlayer::Host => {
                // inform Client that its the turn of Host
                net::send(&mut stream, MessageType::TurnHost);

                // wait for input from Host
                Yellow.with(|| println!("It's your turn!"));
                let mut coord;
                loop {
                    Yellow.with(|| println!("Please enter a valid coordinate: "));
                    coord = util::read_string();
                    if ::model::valid_coordinate(&coord) {
                        break;
                    }
                    Red.with(|| print!("Invalid coordinate! "));
                }
                // modify boards
                let coord_id = Board::get_index(&coord);
                match ::model::match_move(&mut host, &mut client, coord_id) {
                    SubField::Hit => {
                        net::send(&mut stream, MessageType::Hit(coord_id));
                        model::print_boards(&host);
                        current_player = CurrentPlayer::Client;
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
                    Yellow.with(|| println!("Congratulations, you won the game :)"));
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
                let recv: Result<types::MessageType, _> =
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
                                Red.with(|| println!("Unexpected Packet"));
                                net::send(&mut stream, MessageType::Quit);
                                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                                return
                            },
                        }
                    },
                    Err(_) => {
                        Red.with(|| println!("ERROR receiving coord"));
                        net::send(&mut stream, MessageType::Quit);
                        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
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
                        current_player = CurrentPlayer::Host;
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
                    Yellow.with(|| println!("You lost :("));
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
