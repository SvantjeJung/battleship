// Defining types for the network

///////////////////////////
///     Constants       ///
///////////////////////////

pub const LOCALHOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 4200;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Welcome,
    Ping,
    Quit,
    Request,
    RequestShips,
    Login,
    Shoot,
    Board,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msg_type: MessageType,
    pub data: String,
}
