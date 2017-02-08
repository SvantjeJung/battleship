pub mod net;
use ::bincode::serde::{serialize_into, deserialize_from};
use ::bincode;
use ::model::helper;
use self::net::types;
use std::net::{TcpListener, TcpStream};
//use std::thread;

extern crate chan;

pub fn init() {
    let listener = TcpListener::bind((types::LOCALHOST, types::DEFAULT_PORT)).unwrap();

    let (client_conn, _) = listener.accept().unwrap();
    let mut client = client_conn.try_clone().unwrap();

    serialize_into(
        &mut client,
        &(types::Message {
            msg_type: types::MessageType::Welcome,
            data: "Willkommen".to_string() }
        ),
        bincode::SizeLimit::Bounded(2000)
    );

    println!("Hello, world!");
}
