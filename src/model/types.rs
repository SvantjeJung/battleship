use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SubField {
    Water,
    Ship,
    Hit,
    WaterHit,
}

pub struct Player {
    pub own_board: Vec<SubField>,
    pub opponent_board: Vec<SubField>,
}

impl fmt::Display for SubField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubField::Water => write!(f, " ~ "),
            SubField::Ship => write!(f, "\u{2588}\u{2588}\u{2588}"),
            SubField::Hit => write!(f, " X "),
            SubField::WaterHit => write!(f, " O ")
        }
    }
}
