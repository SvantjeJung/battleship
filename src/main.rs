#[macro_use]
extern crate clap;
extern crate term_painter;
extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod client;
mod model;
mod net;
mod server;
mod view;

use bincode::serde::{serialize_into, deserialize_from, DeserializeError};
use clap::AppSettings;
use model::helper;
use model::types::{SubField, Mode};
use std::net::{Shutdown, TcpStream};
use std::{time, thread};
use net::{types};
use std::str;
use net::types::MessageType;
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
    println!("{}", Yellow.paint("Welcome to a round of 'battleship'"));

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
                println!(
                    "{}",
                    Red.paint("NOT IMPLEMENTED: Custom board size will be reset to 10x10")
                );
                match val.parse::<u8>() {
                    Ok(val) => size = val,
                    Err(_) => size = helper::read_usize() as u8,
                }
            }

            let mut board = ::model::types::Board::init();
            if let Some(b) = server_args.value_of("board") {
                board = ::helper::read_extern_board(b);
            }

            if let Some(b) = server_args.value_of("ships") {
                println!("{}", Red.paint("NOT IMPLEMENTED: Custom ship configuration"));
            }

            println!(
                "create server-player: '{}' -- connecting to port: {} -- {2}x{2} board",
                &name,
                port,
                size,
            );

            // create server
            let wait = thread::spawn(move || server::init(name, size, board));
            thread::sleep(time::Duration::from_millis(10));
            wait.join();
        },

        ("client", Some(client_args)) => {
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
            let mut client_conn_clone = connection.try_clone().unwrap();
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            ctrlc::set_handler(move || {
                net::send(&mut client_conn_clone.try_clone().unwrap(), MessageType::Quit);
                client_conn_clone.shutdown(Shutdown::Both).expect("shutdown call failed");
            }).expect("Error setting Ctrl+C handler");

            let mut board = ::model::types::Board::init();
            if let Some(b) = client_args.value_of("board") {
                board = ::helper::read_extern_board(b);
            }

            let mut client = ::model::types::Player {
                own_board: board.clone(),
                op_board: ::model::types::Board::init(),
                player_type: ::model::types::PlayerType::Human,
                capacity: ::model::types::Board::targets(&board),
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
                        println!("RP: {:?}", received);
                        // process_message(received);
                        match received {
                            MessageType::Welcome(msg, host) => {
                                println!("{}", Yellow.paint(msg));
                                host_name = host;
                                // send Login data
                                serialize_into(
                                    &mut connection,
                                    &(types::MessageType::Login(name.to_string())),
                                    bincode::SizeLimit::Infinite
                                );
                            },
                            MessageType::Ping => {
                                // send Ping
                                serialize_into(
                                    &mut connection,
                                    &(types::MessageType::Ping),
                                    bincode::SizeLimit::Infinite
                                );
                            },
                            MessageType::Quit => {
                                println!("Server ended the connection.");
                                break;
                            },
                            MessageType::Board(b) => {
                                // TODO
                                model::print_boards(&b, &vec![SubField::Water; 100]);
                            },
                            MessageType::RequestCoord => {
                                print!("{}", Yellow.paint("It's your turn! "));
                                // send coordinate to shoot
                                let mut coord;
                                loop {
                                    println!(
                                        "{}",
                                        Yellow.paint("Please enter a valid coordinate: ")
                                    );
                                    coord = ::helper::read_string();
                                    if ::model::valid_coordinate(&coord) {
                                        break;
                                    }
                                    print!("{}", Red.paint("Invalid coordinate! "));
                                }

                                net::send(&mut connection, MessageType::Shoot(coord));

                                // receive updated opponent board
                                let result: Result<types::MessageType, DeserializeError> =
                                    deserialize_from(&mut connection, bincode::SizeLimit::Infinite);
                                match result {
                                    Ok(res) => {
                                        match res {
                                            MessageType::Hit(id) => {
                                                println!("{}", Green.paint("Hit!"));
                                                client.op_board[id] = SubField::Hit;
                                            }
                                            MessageType::Miss(id) => {
                                                println!("{}", Blue.paint("Miss!"));
                                                client.op_board[id] = SubField::Miss;
                                            }
                                            _ => {}
                                        }
                                    }
                                    Err(_) => println!("Did not receive Hit or Miss message.")
                                }
                                model::print_boards(&client.own_board, &client.op_board);
                            }
                            MessageType::RequestBoard => {
                                model::print_boards(&client.own_board, &client.op_board);

                                if client.capacity == 0 {
                                    match model::place_ships(&mut client) {
                                        Ok(()) => {},
                                        Err(_) => println!("Failure on placing ships.")
                                    }
                                }

                                // send board
                                net::send_board(&mut connection, &client.own_board);
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
                                ::model::print_boards(&client.own_board, &client.op_board);
                            }
                            MessageType::Lost => {
                                println!("{}", Yellow.paint("You lost the game :("));
                            }
                            MessageType::Won => {
                                println!("{}", Yellow.paint("Congratulations, you won the game!"));
                            }
                            MessageType::Text(t) => {
                                println!("{}", Cyan.paint(t));
                            }
                            MessageType::Unexpected => {
                                // resend expected packet
                            }
                            _ => {
                                println!("{}", Red.paint("Received unexpected packet"));
                            }
                        }
                    },
                    Err(_) => {
                        println!("{}", Red.paint("Connection dropped..."));
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
        _ => {}, // Either no subcommand or one not tested for...
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
            helper::read_usize() as u16
        },
    };

    loop {
        if port < 1024 {
            println!("Please enter a valid port (1024-65535): ");
            port = helper::read_usize() as u16;
        } else {
            break;
        }
    }
    port
}
