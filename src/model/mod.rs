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

/// The actual placement of the ships.
fn place(player: &mut types::Player, ship: types::Ship) -> u8 {

    /* Vector to collect the indices for the possible ship position. */
    let mut indices = Vec::new();

    /* 100 --> invalid value */
    let mut input = 100;
    while input == 100 {
        input = get_input();
        match player.own_board[input] {
            types::SubField::Water => { indices.push(input); },
            _ => { 
                println!("Invalid input, again please.");
                input = 100;
            },
        }    
    }

    println!("{}{}", "Enter 'h' for a horizontal (upwards) orientation of the ship,",
        "'v' for a vertical (rightwards) one.");
    /* x --> invalid value */
    let mut orientation = "x".to_string();
    while orientation == "x" {
        orientation = read_string();
        match orientation.as_ref() {
            "h" => { orientation = "h".to_string(); },
            "v" => { orientation = "v".to_string(); },
            _ => { 
                println!("Invalid input, again please.");
                orientation = "x".to_string();
            }
        }    
    }

    /* Determines the length of the current ship. */
    let len = match ship {
        types::Ship::Carrier => 5,
        types::Ship::Battleship => 4,
        types::Ship::Cruiser => 3,
        types::Ship::Submarine => 3,
        types::Ship::Destroyer => 2,
    };

    if orientation == "v" {
        for _ in 0..len - 1 {
            /* Check to prevent out of bounds errors. */
            if input >= 10 {
                input -= 10;    
            }
            if player.own_board[input] != types::SubField::Water
                /* input < 10 --> no field above */
                || input < 10  {
                    println!("Invalid position for this ship.");
                    println!("Please choose another coordinate.");
                    return 1
            }
            indices.push(input);
        } 
    } else {
        for _ in 0..len - 1 {
            /* Check to prevent out of bounds errors. */
            if input < 99 {
                input += 1;    
            }
            if player.own_board[input] != types::SubField::Water
                /* unit rank == 9 --> no field to the right */
                || (input % 10) == 9 {
                    println!("Invalid position for this ship.");
                    println!("Please choose another coordinate.");
                    return 1
            }
            indices.push(input);
        }
    }

    /* Places the ships on the board. */
    for i in indices.iter() {
        player.own_board[*i] = types::SubField::Ship;
    }

    0
}

/// Handles the initial ship placement for each player.
fn place_ships(mut player1: &mut types::Player, mut player2: &mut types::Player) {

    /* A vector of all the ships each player needs to place. */
    let mut ships = Vec::new();
    ships.push(types::Ship::Carrier);
    ships.push(types::Ship::Battleship);
    ships.push(types::Ship::Battleship);
    ships.push(types::Ship::Cruiser);
    ships.push(types::Ship::Cruiser);
    ships.push(types::Ship::Cruiser);
    ships.push(types::Ship::Submarine);
    ships.push(types::Ship::Submarine);
    ships.push(types::Ship::Submarine);
    ships.push(types::Ship::Submarine);
    ships.push(types::Ship::Destroyer);
    ships.push(types::Ship::Destroyer);
    ships.push(types::Ship::Destroyer);
    ships.push(types::Ship::Destroyer);
    ships.push(types::Ship::Destroyer);

    print_boards(&player1.own_board, &player1.op_board);

    /* Error code, should be replaced by proper error handling later. */
    let mut err;

    /* Asks player1 to place the ships. */
    for i in ships.iter() {
        err = 1;
        while err == 1 {
            println!("Player1, please enter the first coordinate for your {:?}.", i);
            err = place(&mut player1, *i);
            print_boards(&player1.own_board, &player1.op_board);
        } 
    }

    /* Asks player2 to place the ships. */
    for i in ships.iter() {
        err = 1;
        while err == 1 {
            println!("Player2, please enter the first coordinate for your {:?}.", i);
            err = place(&mut player2, *i);
            print_boards(&player2.own_board, &player2.op_board);
        }
    }
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

fn match_move(first: &mut types::Player, second: &mut types::Player, idx: usize) {

    match second.own_board[idx] {
        types::SubField::Water => {
            println!("Miss - try again.");
            first.op_board[idx] = types::SubField::WaterHit;
        }
        types::SubField::Ship => { 
            println!("Hit!");
            first.op_board[idx] = types::SubField::Hit;
            second.own_board[idx] = types::SubField::Hit;
            second.capacity -= 1;
        },
        types::SubField::Hit => { println!("Already hit."); }
        types::SubField::WaterHit => { println!("Miss - try again."); }
    }
}

/// Lets the players perform their moves.
fn make_move(mut first: &mut types::Player, mut second: &mut types::Player) {
    println!("Enter coordinates, {}:", first.name);
    let input = get_input();
    match_move(&mut first, &mut second, input);
}

/// Returns whether all the player's ships got destroyed.
pub fn game_over(player: &types::Player) -> bool {
    player.capacity <= 0
}

pub fn start_match() {
        
    /* Creates the initial (empty) boards (10 x 10) for player1 and player2. */
    let mut player1 = types::Player {
        own_board: vec![types::SubField::Water; 100],
        op_board: vec![types::SubField::Water; 100],
        capacity: 44,
        name: "Player1".to_string(),
    };

    let mut player2 = types::Player {
        own_board: vec![types::SubField::Water; 100],
        op_board: vec![types::SubField::Water; 100],
        capacity: 44,
        name: "Player2".to_string(),
    };

    place_ships(&mut player1, &mut player2);

    loop {
        print_boards(&player1.own_board, &player1.op_board);
        make_move(&mut player1, &mut player2);
        if game_over(&player2) {
            println!("G A M E   O V E R");
            println!("Congratulations, Player1");
            break;
        }

        print_boards(&player2.own_board, &player2.op_board);
        make_move(&mut player2, &mut player1);
        if game_over(&player1) {
            println!("G A M E   O V E R");
            println!("Congratulations, Player2");
            break;
        }        
    }
}
