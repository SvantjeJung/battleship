extern crate bincode;
#[macro_use]
extern crate clap;
extern crate ctrlc;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate term_painter;

mod client;
mod model;
mod net;
mod server;
mod util;
mod view;

use bincode::serde::{deserialize_from, DeserializeError};
use clap::AppSettings;
use model::types::{Board, SubField, Mode};
use net::types::{self, MessageType};
use std::net::{Shutdown, TcpStream};
use term_painter::ToStyle;
use term_painter::Color::*;

const BOARD_SIZE: u8 = 10;

fn main() {
    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///                                Command Line Interface                                   ///
    ///////////////////////////////////////////////////////////////////////////////////////////////
    let battleship = clap_app!(battleship =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Two player battleship game")
        (@subcommand server =>
            (about: "Server instance for the game")
            (version: crate_version!())
            (author: crate_authors!())
            (@arg port: +required +takes_value "Connect to <port> on localhost")
            (@arg name: +required +takes_value "Name of player")
            (@arg size: -s --size +takes_value
                "set N as board dimension => N x N [not yet implemented]"
            )
            (@arg ships: --ships +takes_value "load ship configuration [not yet implemented]")
            (@arg board: --board +takes_value "load board configuration")
        )
        (@subcommand client =>
            (about: "Client instance for the game")
            (version: crate_version!())
            (author: crate_authors!())
            (@arg ip: +required +takes_value "Connect to address")
            (@arg port: +required +takes_value "Connect to port")
            (@arg name: +required +takes_value "Name of player")
            (@arg board: --board +takes_value "load board configuration")
        )
        (@subcommand single =>
            (about: "Play against the computer")
            (version: crate_version!())
            (author: crate_authors!())
            (@arg name: +required +takes_value "Name of player")
        )
    )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///                                         Start                                           ///
    ///////////////////////////////////////////////////////////////////////////////////////////////
    Yellow.with(|| println!("Welcome to a round of 'battleship'"));

    // default --> player vs. player
    let mut mode = Mode::PvP;

    match battleship.subcommand() {
        ("server", Some(server_args)) => {
            // required arguments
            let port = validate_port(server_args.value_of("port").unwrap());
            let name = String::from(server_args.value_of("name").unwrap());

            // optional arguments
            let mut size = BOARD_SIZE;
            if let Some(val) = server_args.value_of("size") {
                Red.with(|| println!("NOT IMPLEMENTED: Custom board size will be reset to 10x10"));
                match val.parse::<u8>() {
                    Ok(val) => size = val,
                    Err(_) => size = util::read_usize() as u8,
                }
            }

            let mut board = Board::init();
            if let Some(b) = server_args.value_of("board") {
                board = util::read_extern_board(b);
            }

            if let Some(_) = server_args.value_of("ships") {
                Red.with(|| println!("NOT IMPLEMENTED: Custom ship configuration"));
            }

            println!(
                "create server-player: '{}' -- connecting to port: {} -- {2}x{2} board",
                &name,
                port,
                size,
            );

            // create server
            let server = server::Server {
                ip: net::types::LOCALHOST,
                port: port,
                host_name: name,
                host_board: board,
                board_dim: size,
            };

            server::init(server);
        },

        ("client", Some(client_args)) => {
            // required arguments
            let ip = client_args.value_of("ip").unwrap();
            // TODO: check for valid ip-address
            let port = validate_port(client_args.value_of("port").unwrap());
            let name = client_args.value_of("name").unwrap();

            println!(
                "create client-player: '{}' -- connecting to {}:{}",
                name,
                ip,
                port,
            );

            // create client instance and connect to server
            let mut connection = TcpStream::connect((ip, port)).unwrap();
            // add CTRL+C system hook, so that connection partner is informed about disconnect
            let client_conn_clone = connection.try_clone().unwrap();
            ctrlc::set_handler(move || {
                net::send(&mut client_conn_clone.try_clone().unwrap(), MessageType::Quit);
                client_conn_clone.shutdown(Shutdown::Both).expect("shutdown call failed");
            }).expect("Error setting Ctrl+C handler");

            let mut board = Board::init();
            if let Some(b) = client_args.value_of("board") {
                board = util::read_extern_board(b);
            }

            let mut client = ::model::types::Player {
                own_board: board.clone(),
                op_board: Board::init(),
                player_type: ::model::types::PlayerType::Human,
                capacity: Board::targets(&board),
                name: name.to_string(),
            };

            let mut host_name = "SERVER".to_string();
            loop {
                //let mut buffer = Vec::new();
                //let msg = client_connection.read_to_end(&mut buffer);
                //println!("{}", str::from_utf8(&buffer).unwrap());
                let recv: Result<types::MessageType, DeserializeError> =
                    deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
                match recv {
                    Ok(received) => {
                        // process_message(received);
                        match received {
                            MessageType::Welcome(msg, host) => {
                                Yellow.with(|| println!("{}", (msg)));
                                host_name = host;
                                net::send(&mut connection, MessageType::Login(name.to_string()));
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
                                let result: Result<types::MessageType, DeserializeError> =
                                    deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
                                match result {
                                    Ok(res) => {
                                        match res {
                                            MessageType::Hit(id) => {
                                                Green.with(|| println!("Hit!"));
                                                client.op_board[id] = SubField::Hit;
                                            }
                                            MessageType::Miss(id) => {
                                                Blue.with(|| println!("Miss!"));
                                                client.op_board[id] = SubField::Miss;
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
                                    match model::place_ships(&mut client) {
                                        Ok(()) => {},
                                        Err(_) => println!("Failure on placing ships.")
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

                                let result: Result<types::MessageType, DeserializeError> =
                                    deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
                                match result {
                                    Ok(res) => {
                                        match res {
                                            MessageType::Hit(id) => {
                                                client.own_board[id] = SubField::Hit;
                                            }
                                            MessageType::Miss(id) => {
                                                client.own_board[id] = SubField::Miss;
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
        },

        ("single", Some(single_args)) => {
            let name = single_args.value_of("name").unwrap();

            println!(
                "create player: '{}'",
                name
            );

            // TODO: create game instance + AI
            println!("--- Single-Player-Mode ---");
            mode = Mode::Single;
        },
        _ => unimplemented!()
    }

    //model::start_round(mode);
    println!("");
}

/// Validate port
/// Only allow usage of ports 1024 up to 65535
fn validate_port(p: &str) -> u16 {
    let mut port = match p.parse::<u16>() {
        Ok(p) => p,
        Err(_) => {
            println!("Please enter a valid port (1024-65535): ");
            util::read_usize() as u16
        },
    };

    loop {
        if port < 1024 {
            println!("Please enter a valid port (1024-65535): ");
            port = util::read_usize() as u16;
        } else {
            break;
        }
    }
    port
}

/// Validate port
/// Only allow usage of ports from 1024 up to 65535
/// For clap_app! usage if someone knew how to add this to the macro-call...
fn valid_port(p: &str) -> Result<(), String> {
    let msg = "Please choose a valid port (1024-65535)".to_string();
    match p.parse::<u16>() {
        Ok(port) => {
            if port >= 1024 {
                Ok(())
            } else {
                Err(msg)
            }
        }
        Err(_) => Err(msg)
    }
}
