pub mod types;

use ::bincode::serde::serialize_into;
use std::net::TcpStream;

/// Send a message to connected partner on stream
pub fn send(mut stream: &mut TcpStream, msg: types::MessageType) {
    let _ = serialize_into(
        &mut stream,
        &msg,
        ::bincode::SizeLimit::Infinite
    );
}
