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

use clap::AppSettings;
use model::types::{Board};
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

    match battleship.subcommand() {
        ("server", Some(server_args)) => {
            // required arguments
            let port = validate_port(server_args.value_of("port").unwrap());
            let name = server_args.value_of("name").unwrap().to_string();

            // optional arguments
            let mut size = BOARD_SIZE;
            if let Some(val) = server_args.value_of("size") {
                Red.with(|| println!("NOT IMPLEMENTED: Custom board size will be reset to 10x10"));
                match val.parse::<u8>() {
                    Ok(val) => size = val,
                    Err(_) => size = util::read_usize() as u8,
                }
            }

            let board = server_args.value_of("board")
                .map(|b| util::read_extern_board(b))
                .unwrap_or(Board::init());

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

            let board = client_args.value_of("board")
                .map(|b| util::read_extern_board(b))
                .unwrap_or(Board::init());

            let client = ::model::types::Player {
                own_board: board.clone(),
                op_board: Board::init(),
                player_type: ::model::types::PlayerType::Human,
                capacity: Board::targets(&board),
                name: name.to_string(),
            };

            // connect to server
            client::connect(client, ip, port);
        },

        ("single", Some(single_args)) => {
            let name = single_args.value_of("name").unwrap();

            println!(
                "create player: '{}'",
                name
            );

            // TODO: create game instance + AI
            println!("--- Single-Player-Mode ---");
            model::start_round();
        },
        _ => unimplemented!()
    }

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
