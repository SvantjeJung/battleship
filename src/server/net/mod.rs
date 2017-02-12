pub mod types;

use ::bincode::serde::{serialize_into, deserialize_from, DeserializeError};
use ::model::types::SubField;
use std::net::{TcpListener, TcpStream};

pub fn send_board(mut stream: &mut TcpStream, board: &Vec<SubField>) {
    serialize_into(
        &mut stream,
        &(types::MessageType::Board(board.clone())),
        ::bincode::SizeLimit::Infinite
    );
}

pub fn send(mut stream: &mut TcpStream, msg: types::MessageType) {
    serialize_into(
        &mut stream,
        &msg,
        ::bincode::SizeLimit::Infinite
    );
}