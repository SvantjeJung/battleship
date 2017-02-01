use term_painter::ToStyle;
use term_painter::Color::*;
use std::io;

mod types;

// Game logic (board, initialization, valid move, set, play or finished)

/// Visualization of the boards.
fn print_boards(board1: &Vec<types::SubField>, board2: &Vec<types::SubField>) {
    println!("{}{}", "------------------ O W N   B O A R D ------------------",
        "------------------------- O P P O N E N T ----------------");

    let mut cnt = 9;

    for i in 0..10 {
        println!("");
        print!("{}  ", cnt);
        for j in 0..10 {
            if board1[j + i * 10] == types::SubField::Hit {
                print!(" {} ", Red.paint(board1[j + i * 10]));    
            } else {
                print!(" {} ", board1[j + i * 10]);
            }
        }
        print!("        ");
        print!("{}  ", cnt);
        for k in 0..10 {
            if board2[k + i * 10] == types::SubField::Hit {
                print!(" {} ", Green.paint(board2[k + i * 10]));    
            } else {
                print!(" {} ", board2[k + i * 10]);
            }
        }
        println!("");
        cnt -= 1;
    }
    println!("");
    println!("{}{}", "     A    B    C    D    E    F    G    H    I    J",
        "               A    B    C    D    E    F    G    H    I    J");
    println!("");
    println!("{}{}", "----------------------------------------------------",
        "--------------------------------------------------------------");
}

/// Reads a string from the terminal / user.
fn read_string() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    /* Discard trailing newline */
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Just an example for now..
fn place_ships(player1: &mut types::Player, player2: &mut types::Player) {
    /* Ship 1 */
    player1.own_board[20] = types::SubField::Ship;
    player1.own_board[10] = types::SubField::Ship;
    player1.own_board[0] = types::SubField::Ship;
    /* Ship 2 */
    player1.own_board[65] = types::SubField::Ship;
    player1.own_board[66] = types::SubField::Ship;
    player1.own_board[67] = types::SubField::Ship;
    player1.own_board[68] = types::SubField::Ship;
    /* Ship 3 */
    player1.own_board[90] = types::SubField::Ship;
    player1.own_board[91] = types::SubField::Ship;
    player1.own_board[80] = types::SubField::Ship;
    player1.own_board[81] = types::SubField::Ship;
    /* Ship 4 */
    player1.own_board[5] = types::SubField::Ship;
    player1.own_board[15] = types::SubField::Ship;
    player1.own_board[25] = types::SubField::Ship;
    player1.own_board[35] = types::SubField::Ship;

    /* Ship 1 */
    player2.own_board[40] = types::SubField::Ship;
    player2.own_board[50] = types::SubField::Ship;
    player2.own_board[60] = types::SubField::Ship;
    /* Ship 2 */
    player2.own_board[71] = types::SubField::Ship;
    player2.own_board[72] = types::SubField::Ship;
    player2.own_board[73] = types::SubField::Ship;
    player2.own_board[74] = types::SubField::Ship;
    /* Ship 3 */
    player2.own_board[99] = types::SubField::Ship;
    player2.own_board[98] = types::SubField::Ship;
    player2.own_board[97] = types::SubField::Ship;
    player2.own_board[96] = types::SubField::Ship;
    /* Ship 4 */
    player2.own_board[1] = types::SubField::Ship;
    player2.own_board[2] = types::SubField::Ship;
    player2.own_board[3] = types::SubField::Ship;
    player2.own_board[4] = types::SubField::Ship;
}

/// Reads the coordinates of a field from the user
/// and returns the corresponding index.
fn get_input() -> usize {

    /* 100 --> invalid */
    let mut input = 100;

    while input == 100 {
        input = match read_string().as_ref() {
            "A0" => 90, "A1" => 80,"A2" => 70, "A3" => 60, "A4" => 50,
            "A5" => 40, "A6" => 30, "A7" => 20, "A8" => 10, "A9" => 0,
            "B0" => 91, "B1" => 81, "B2" => 71, "B3" => 61, "B4" => 51,
            "B5" => 41, "B6" => 31, "B7" => 21, "B8" => 11, "B9" => 1,
            "C0" => 92, "C1" => 82, "C2" => 72, "C3" => 62, "C4" => 52,
            "C5" => 42, "C6" => 32, "C7" => 22, "C8" => 12, "C9" => 2,
            "D0" => 93, "D1" => 83, "D2" => 73, "D3" => 63, "D4" => 53,
            "D5" => 43, "D6" => 33, "D7" => 23, "D8" => 13, "D9" => 3,
            "E0" => 94, "E1" => 84, "E2" => 74, "E3" => 64, "E4" => 54,
            "E5" => 44, "E6" => 34, "E7" => 24, "E8" => 14, "E9" => 4,
            "F0" => 95, "F1" => 85, "F2" => 75, "F3" => 65, "F4" => 55,
            "F5" => 45, "F6" => 35, "F7" => 25, "F8" => 15, "F9" => 5,
            "G0" => 96, "G1" => 86, "G2" => 76, "G3" => 66, "G4" => 56,
            "G5" => 46, "G6" => 36, "G7" => 26, "G8" => 16, "G9" => 6,
            "H0" => 97, "H1" => 87, "H2" => 77, "H3" => 67, "H4" => 57,
            "H5" => 47, "H6" => 37, "H7" => 27, "H8" => 17, "H9" => 7,
            "I0" => 98, "I1" => 88, "I2" => 78, "I3" => 68, "I4" => 58,
            "I5" => 48, "I6" => 38, "I7" => 28, "I8" => 18, "I9" => 8,
            "J0" => 99, "J1" => 89, "J2" => 79, "J3" => 69, "J4" => 59,
            "J5" => 49, "J6" => 39, "J7" => 29, "J8" => 19, "J9" => 9,
            _ => 100,
        };

        if input == 100 {
            println!("Invalid field, try again.")
        }
    }
    input
}

fn match_move(idx: usize, own_board: &mut Vec<types::SubField>, op_board: &mut Vec<types::SubField>) {

    match op_board[idx] {
        types::SubField::Water => {
            println!("Miss - try again.");
            own_board[idx] = types::SubField::WaterHit;
        }
        types::SubField::Ship => { 
            println!("Hit!");
            op_board[idx] = types::SubField::Hit;
            own_board[idx] = types::SubField::Hit;
        },
        types::SubField::Hit => { println!("Already hit."); }
        types::SubField::WaterHit => { println!("Miss - try again."); }
    }
}

/// Lets the players perform their moves.
fn make_move(p1: bool, own_board: &mut Vec<types::SubField>, op_board: &mut Vec<types::SubField>) {

    if p1 {
        println!("Player1: ");
        let input = get_input();
        match_move(input, own_board, op_board);
    } else {
        println!("Player2: ");
        let input = get_input();
        match_move(input, own_board, op_board);
    }
}

pub fn start_match() {
        
    /* Creates the initial (empty) boards (10 x 10) for player1 and player2. */
    let mut player1 = types::Player {
        own_board: vec![types::SubField::Water; 100],
        opponent_board: vec![types::SubField::Water; 100],
    };

    let mut player2 = types::Player {
        own_board: vec![types::SubField::Water; 100],
        opponent_board: vec![types::SubField::Water; 100],
    };

    place_ships(&mut player1, &mut player2);

    loop {
        print_boards(&player1.own_board, &player1.opponent_board);
        make_move(true, &mut player1.opponent_board, &mut player2.own_board);
        // check if game over

        print_boards(&player2.own_board, &player2.opponent_board);
        make_move(false, &mut player2.opponent_board, &mut player1.own_board);
        // check if game over
    }
}
