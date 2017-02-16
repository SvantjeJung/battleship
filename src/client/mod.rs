use bincode;
use bincode::serde::deserialize_from;
use ctrlc;
use model;
use model::types::{Player, SubField};
use net;
use net::types::{self, MessageType};
use std::net::{Shutdown, TcpStream};
use term_painter::ToStyle;
use term_painter::Color::*;
use util;

/// Connecting the player to IP:port
pub fn connect(client: Player, ip: &str, port: u16) {
    // create client instance and connect to server
    let connection = TcpStream::connect((ip, port)).unwrap();

    // add CTRL+C system hook, so that connection partner is informed about disconnect
    let client_conn_clone = connection.try_clone().unwrap();
    ctrlc::set_handler(move || {
        net::send(&mut client_conn_clone.try_clone().unwrap(), MessageType::Quit);
        client_conn_clone.shutdown(Shutdown::Both).expect("shutdown call failed");
    }).expect("Error setting Ctrl+C handler");

    play(connection, client);
}

/// Actual game flow
fn play(mut connection: TcpStream, mut client: Player) {
    let mut host_name = "SERVER".to_string();
    loop {
        let recv: Result<types::MessageType, _> =
            deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
        match recv {
            Ok(received) => {
                // process_message(received);
                match received {
                    MessageType::Welcome(msg, host) => {
                        Yellow.with(|| println!("{}", (msg)));
                        host_name = host;
                        net::send(&mut connection, MessageType::Login(client.name.clone()));
                    },
                    MessageType::Ping => {
                        net::send(&mut connection, MessageType::Ping);
                    },
                    MessageType::Quit => {
                        println!("Server ended the connection.");
                        break;
                    },
                    MessageType::RequestCoord => {
                        Yellow.with(|| print!("It's your turn! "));
                        // send coordinate to shoot
                        let mut coord;
                        loop {
                            Yellow.with(|| println!("Please enter a valid coordinate: "));
                            coord = util::read_string();
                            if ::model::valid_coordinate(&coord) {
                                break;
                            }
                            Red.with(|| print!("Invalid coordinate! "));
                        }

                        net::send(&mut connection, MessageType::Shoot(coord));

                        // receive updated opponent board
                        let result: Result<types::MessageType, _> =
                            deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
                        match result {
                            Ok(res) => {
                                match res {
                                    MessageType::Hit(id) => {
                                        let row = id / 10;
                                        let col = id % 10;
                                        Green.with(|| println!("Hit!"));
                                        client.op_board[row][col] = SubField::Hit;
                                    }
                                    MessageType::Miss(id) => {
                                        let row = id / 10;
                                        let col = id % 10;
                                        Blue.with(|| println!("Miss!"));
                                        client.op_board[row][col] = SubField::Miss;
                                    }
                                    _ => {}
                                }
                            }
                            Err(_) => println!("Did not receive Hit or Miss message.")
                        }
                        model::print_boards(&client);
                    }
                    MessageType::RequestBoard => {
                        model::print_boards(&client);

                        if client.capacity == 0 {
                            loop {
                                match model::place_ships(&mut client) {
                                    Ok(()) => { break; },
                                    Err(model::types::ErrorType::DeadEndHuman) => {
                                        Red.with(|| println!(
                                            "No suitable position left, {}",
                                            "please restart the ship placement.")
                                        );
                                        model::restart_placement(&mut client);
                                    },
                                    Err(_) => {
                                        Red.with(|| println!("Failed placing ships!"));
                                    },
                                }
                            }
                        }

                        // send board
                        net::send(
                            &mut connection,
                            MessageType::Board(client.own_board.clone())
                        );
                    }
                    MessageType::Text(t) => {
                        Cyan.with(|| println!("{}", t));
                    }
                    MessageType::TurnHost => {
                        println!(
                            "{} {} {}",
                            Cyan.paint("Wait for"),
                            Yellow.paint(&host_name),
                            Cyan.paint("to finish turn!"),
                        );

                        let result: Result<types::MessageType, _> =
                            deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
                        match result {
                            Ok(res) => {
                                match res {
                                    MessageType::Hit(id) => {
                                        let row = id / 10;
                                        let col = id % 10;
                                        client.own_board[row][col] = SubField::Hit;
                                    }
                                    MessageType::Miss(id) => {
                                        let row = id / 10;
                                        let col = id % 10;
                                        client.own_board[row][col] = SubField::Miss;
                                    }
                                    _ => {}
                                }
                            }
                            Err(_) => println!("Did not receive Hit or Miss message.")
                        }
                        model::print_boards(&client);
                    }
                    MessageType::Unexpected => {
                        Red.with(|| println!("Handshake done wrong!"));
                    }
                    MessageType::Lost => {
                        Yellow.with(|| println!("You lost the game :("));
                    }
                    MessageType::Won => {
                        Yellow.with(|| println!("Congratulations, you won the game!"));
                    }
                    _ => {
                        Red.with(|| println!("Received unexpected packet"));
                    }
                }
            },
            Err(_) => {
                Red.with(|| println!("Connection dropped..."));
                break;
            },
        };
    }
}
