// Defining types for the network

use ::model::types::{SubField, Player};

///////////////////////////
///     Constants       ///
///////////////////////////

pub const LOCALHOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 4200;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Lost,
    Ping,
    Quit,
    Ready,
    RequestBoard,
    RequestCoord,
    TurnClient,
    TurnHost,
    Unexpected,
    Won,
    Hit(usize),
    Login(String),
    Miss(usize),
    Welcome(String, String),
    Shoot(String),
    Text(String),
    Board(Vec<SubField>),
}
