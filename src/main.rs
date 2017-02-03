#[macro_use]
extern crate clap;
extern crate term_painter;
extern crate rand;

mod client;
mod model;
mod server;
mod view;

use clap::AppSettings;
use model::{helper, types};

const BOARD_SIZE: u8 = 10;

fn main() {
    // initialize the CLI
    let battleship = clap_app!(battleship =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Two player battleship game")
        (@subcommand server =>
            (about: "Server instance for the game")
            (version: crate_version!())
            (author: crate_authors!())
            (@arg port: +required +takes_value "listening port")
            (@arg name: +required +takes_value "Name of player")
            (@arg size: -s --size +takes_value
                "set N as board dimension => N x N [not yet implemented]"
            )
            (@arg ships: --ships +takes_value "load ship configuration [not yet implemented]")
        )
        (@subcommand client =>
            (about: "Client instance for the game")
            (version: crate_version!())
            (author: crate_authors!())
            (@arg ip: +required +takes_value "Connect to address")
            (@arg port: +required +takes_value "Connect to port")
            (@arg name: +required +takes_value "Name of player")
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

    println!("Welcome to a round of 'battleship'");
    /* Default --> player vs. player */
    let mut mode = types::Mode::PvP;

    match battleship.subcommand() {
        ("server", Some(server_args)) => {
            // required arguments
            let port = validate_port(server_args.value_of("port").unwrap());
            let name = server_args.value_of("name").unwrap();

            // optional arguments
            let mut size = BOARD_SIZE;
            if let Some(val) = server_args.value_of("size") {
                match val.parse::<u8>() {
                    Ok(val) => size = val,
                    Err(_) => size = helper::read_usize() as u8,
                }
            }

            if let Some(_) = server_args.value_of("ships") {
                println!("NOT IMPLEMENTED: loading custom ship configuration");
            }

            println!(
                "create server-player: '{}' -- listening on port: {} -- {}x{} board",
                name,
                port,
                size,
                size,
            );

            // TODO: create server instance
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

            // TODO: create client instance
        },
        ("single", Some(single_args)) => {
            let name = single_args.value_of("name").unwrap();

            println!(
                "create player: '{}'",
                name
            );

            println!("--- Single-Player-Mode ---");
            mode = types::Mode::Single;
        },
        _ => {}, // Either no subcommand or one not tested for...
    }

    model::start_round(mode);
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
