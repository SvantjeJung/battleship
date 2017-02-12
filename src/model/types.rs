use std::fmt;

#[derive(PartialEq)]
pub enum Mode {
    PvP,
    Single,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerType {
    Human,
    DumbAI,
    SmartAI,
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SubField {
    Water,
    Ship,
    Hit,
    WaterHit,
}

pub enum ErrorType {
    DeadEnd,
    InvalidField,
}

pub struct ShipType {
    pub name: String,
    pub size: usize,
    pub amount: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub own_board: Vec<SubField>,
    pub op_board: Vec<SubField>,
    pub player_type: PlayerType,
    // The "life" basically - the amount of hits necessary
    // for the opponent to win the game.
    pub capacity: usize,
    pub name: String,
}

impl fmt::Display for SubField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubField::Water => write!(f, " ~ "),
            SubField::Ship => write!(f, " ⛵ "),
            SubField::Hit => write!(f, " ❌ "),
            SubField::WaterHit => write!(f, " ○ ")
        }
    }
}
