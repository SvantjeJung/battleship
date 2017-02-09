// Defining types for the network

use ::model::types::SubField;

///////////////////////////
///     Constants       ///
///////////////////////////

pub const LOCALHOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 4200;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Ping,
    Quit,
    Ready,
    RequestBoard,
    RequestCoord,
    Unexpected,
    Login(String),
    Welcome(String),
    Shoot(String),
    Board(Vec<SubField>),
}
