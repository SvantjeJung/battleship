pub mod net;
use ::bincode::serde::{serialize_into, deserialize_from, DeserializeError};
use ::bincode;
use ::model::{helper};
use ::model::types::{Player, SubField};
use self::net::types;
use self::net::types::{MessageType};
use std::net::{TcpListener, TcpStream};
//use std::thread;

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
        &(types::MessageType::Welcome("Willkommen".to_string())),
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
        name: name,
        capacity: 0,
    };

    let client = Player {
        own_board: vec![SubField::Water; 100],
        op_board: vec![SubField::Water; 100],
        name: client_name,
        capacity: 0,
    };

    // start game
    start(host, client, client_stream);

    println!("Bye.");
}

fn start(host: Player, client: Player, mut stream: TcpStream) {
    // ask for ship placements, simultaneously
    // wait for both threads to end
    //let wait_for_client = thread::spawn();
    //wait.join();
    serialize_into(
        &mut stream,
        &(types::MessageType::Board(client.own_board.clone())),
        bincode::SizeLimit::Infinite
    );

    // Just testing Coordinate Request and Receive ...
    serialize_into(
        &mut stream,
        &(types::MessageType::RequestCoord),
        bincode::SizeLimit::Infinite
    );
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

    // choose start player
    let current_player = match choose_player(&host, &client).name == host.name {
        true => CurrentPlayer::Host,
        false => CurrentPlayer::Client,
    };
    println!("Starting player: {:?}", current_player);

    // while game not ended: take turn
    loop {
        match current_player {
            CurrentPlayer::Host => {
                // inform Client that its the turn of Host
                // wait for input from Host
                // modify boards
                // send client.own_board to Client + plus message which Field was Hit/Miss
                // match Hit | Miss | Destroyed
                // if Host won: send message to Client, end game
                break;
            }
            CurrentPlayer::Client => {
                // inform Client that its his turn
                // wait for input from Client
                // modify boards
                // send client.op_board to Client + message which Field Hit/Miss
                // match Hit | Miss | Destroyed
                // if Client won: send message to Client, end game
                break;
            }
        }
    }
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
