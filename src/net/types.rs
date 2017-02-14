use model::types::SubField;

///////////////////////////
///     Constants       ///
///////////////////////////
pub const LOCALHOST: &'static str = "127.0.0.1";

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
    Board(Vec<SubField>),
    Hit(usize),
    Login(String),
    Miss(usize),
    Shoot(String),
    Text(String),
    Welcome(String, String),
}
