use std::fmt;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerType {
    Human,
    AI,
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SubField {
    Water,
    Ship,
    Hit,
    Miss,
}

pub enum ErrorType {
    DeadEndHuman,
    DeadEndAI,
    InvalidField,
}

pub struct ShipType {
    pub name: String,
    pub size: usize,
    pub amount: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub own_board: [[SubField; 10]; 10],
    pub op_board: [[SubField; 10]; 10],
    pub player_type: PlayerType,
    // The "life" basically - the amount of hits necessary
    // for the opponent to win the game.
    pub capacity: usize,
    pub name: String,
}

impl Player {
    pub fn set_board(&mut self, b: [[SubField; 10]; 10]) {
        self.own_board = b.clone();
        self.capacity = Board::targets(&b);
    }
}

impl fmt::Display for SubField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubField::Water => write!(f, " ~ "),
            SubField::Ship => write!(f, " ⛵ "),
            SubField::Hit => write!(f, " ❌ "),
            SubField::Miss => write!(f, " ○ ")
        }
    }
}

pub struct Board;

impl Board {
    /// Initializes a board with just water
    pub fn init() -> [[SubField; 10]; 10] {
        [[SubField::Water; 10]; 10]
    }

    /// Returns index on board for given inputs
    pub fn get_index(coord: &str) -> usize {
        let case = coord.to_uppercase();
        match case.as_ref() {
            "A0"|"0A" => 90, "A1"|"1A" => 80, "A2"|"2A" => 70, "A3"|"3A" => 60, "A4"|"4A" => 50,
            "A5"|"5A" => 40, "A6"|"6A" => 30, "A7"|"7A" => 20, "A8"|"8A" => 10, "A9"|"9A" => 0,
            "B0"|"0B" => 91, "B1"|"1B" => 81, "B2"|"2B" => 71, "B3"|"3B" => 61, "B4"|"4B" => 51,
            "B5"|"5B" => 41, "B6"|"6B" => 31, "B7"|"7B" => 21, "B8"|"8B" => 11, "B9"|"9B" => 1,
            "C0"|"0C" => 92, "C1"|"1C" => 82, "C2"|"2C" => 72, "C3"|"3C" => 62, "C4"|"4C" => 52,
            "C5"|"5C" => 42, "C6"|"6C" => 32, "C7"|"7C" => 22, "C8"|"8C" => 12, "C9"|"9C" => 2,
            "D0"|"0D" => 93, "D1"|"1D" => 83, "D2"|"2D" => 73, "D3"|"3D" => 63, "D4"|"4D" => 53,
            "D5"|"5D" => 43, "D6"|"6D" => 33, "D7"|"7D" => 23, "D8"|"8D" => 13, "D9"|"9D" => 3,
            "E0"|"0E" => 94, "E1"|"1E" => 84, "E2"|"2E" => 74, "E3"|"3E" => 64, "E4"|"4E" => 54,
            "E5"|"5E" => 44, "E6"|"6E" => 34, "E7"|"7E" => 24, "E8"|"8E" => 14, "E9"|"9E" => 4,
            "F0"|"0F" => 95, "F1"|"1F" => 85, "F2"|"2F" => 75, "F3"|"3F" => 65, "F4"|"4F" => 55,
            "F5"|"5F" => 45, "F6"|"6F" => 35, "F7"|"7F" => 25, "F8"|"8F" => 15, "F9"|"9F" => 5,
            "G0"|"0G" => 96, "G1"|"1G" => 86, "G2"|"2G" => 76, "G3"|"3G" => 66, "G4"|"4G" => 56,
            "G5"|"5G" => 46, "G6"|"6G" => 36, "G7"|"7G" => 26, "G8"|"8G" => 16, "G9"|"9G" => 6,
            "H0"|"0H" => 97, "H1"|"1H" => 87, "H2"|"2H" => 77, "H3"|"3H" => 67, "H4"|"4H" => 57,
            "H5"|"5H" => 47, "H6"|"6H" => 37, "H7"|"7H" => 27, "H8"|"8H" => 17, "H9"|"9H" => 7,
            "I0"|"0I" => 98, "I1"|"1I" => 88, "I2"|"2I" => 78, "I3"|"3I" => 68, "I4"|"4I" => 58,
            "I5"|"5I" => 48, "I6"|"6I" => 38, "I7"|"7I" => 28, "I8"|"8I" => 18, "I9"|"9I" => 8,
            "J0"|"0J" => 99, "J1"|"1J" => 89, "J2"|"2J" => 79, "J3"|"3J" => 69, "J4"|"4J" => 59,
            "J5"|"5J" => 49, "J6"|"6J" => 39, "J7"|"7J" => 29, "J8"|"8J" => 19, "J9"|"9J" => 9,
            _ => 100,
        }
    }

    /// Returns true if no Ships set on board
    pub fn empty(board: &[[SubField; 10]; 10]) -> bool {
        let mut empty = true;
        for i in 0..10 {
            if !empty {
                return empty
            }
            empty = board[i].iter().all(|elem| *elem == SubField::Water);
        }
        empty
    }

    /// Returns number of Ships on board
    pub fn targets(board: &[[SubField; 10]; 10]) -> usize {
        let mut cnt = 0;
        for i in 0..10 {
            cnt += board[i].iter().filter(|&elem| *elem == SubField::Ship).count();
        }
        cnt
    }
}
