pub mod types;

use bincode::serde::serialize_into;
use bincode::SizeLimit;
use std::net::TcpStream;

/// Send a message to connected partner on stream
pub fn send(mut stream: &mut TcpStream, msg: types::MessageType) {
    serialize_into(
        stream,
        &msg,
        SizeLimit::Infinite
    ).unwrap();
}
