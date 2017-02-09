pub mod net;
use ::bincode::serde::{serialize_into, deserialize_from, DeserializeError};
use ::bincode;
use ::model::{helper};
use ::model::types::{Player, SubField};
use self::net::types;
use self::net::types::{Message, MessageType};
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
    let mut client = client_conn.try_clone().unwrap();

    // welcome client
    serialize_into(
        &mut client,
        &(types::Message {
            msg_type: types::MessageType::Welcome,
            data: "Willkommen".to_string() }
        ),
        bincode::SizeLimit::Bounded(2000)
    );

    // getLogin
    let recv: Result<types::Message, DeserializeError> =
    deserialize_from(&mut client, bincode::SizeLimit::Infinite);
    let client_name = match recv {
        Ok(received) => {
            println!("{:?}: {}", received.msg_type, received.data);
            match received.msg_type {
                MessageType::Login => {
                    received.data
                },
                _ => "".to_string(),
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
    start(host, client);

    println!("Bye.");
}

fn start(host: Player, client: Player) {
    // ask for ship placements, simultaneously
    // wait for both threads to end

    // choose start player
    let current_player = match choose_player(&host, &client).name == host.name {
        true => CurrentPlayer::Host,
        false => CurrentPlayer::Client,
    };
    println!("Starting player: {:?}", current_player);


    // while game not ended: take turn
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
