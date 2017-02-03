use term_painter::ToStyle;
use term_painter::Color::*;

pub mod types;
pub mod helper;

// Game logic (board, initialization, valid move, set, play or finished)

/// Visualization of the boards.
fn print_boards(board1: &Vec<types::SubField>, board2: &Vec<types::SubField>) {
    println!("{}{}", "------------------ O W N   B O A R D ------------------",
        "-------------------------- O P P O N E N T ---------------");

    let mut cnt = 9;

    for i in 0..10 {
        print!("\n{0:<width$}", cnt, width=3);
        for j in 0..10 {
            if board1[j + i * 10] == types::SubField::Hit {
                print!(" {} ", Red.paint(board1[j + i * 10]));
            } else {
                print!(" {} ", board1[j + i * 10]);
            }
        }
        print!("        ");
        print!("{}  ", cnt);
        for j in 0..10 {
            if board2[j + i * 10] == types::SubField::Hit {
                print!(" {} ", Green.paint(board2[j + i * 10]));
            } else {
                print!(" {} ", board2[j + i * 10]);
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
        "-------------------------------------------------------------");
}

/// The actual placement of the ships.
fn place(player: &mut types::Player, ship: &types::ShipType) -> Result<(), String> {

    /* Vector to collect the indices for the possible ship position. */
    let mut indices = Vec::new();

    let mut input = get_input();
    loop {
        match player.own_board[input] {
            types::SubField::Water => { indices.push(input); break; },
            _ => {
                println!("Invalid input, again please.");
                input = get_input();
            },
        }
    }

    println!(
    "Enter 'h' for a horizontal (rightwards) orientation of the ship,
    'v' for a vertical (upwards) one."
    );

    let mut ori = helper::read_string();
    loop {
        match ori.as_str() {
            "h" | "v" => break,
            _ => {
                println!("Invalid input, again please.");
                ori = helper::read_string();
            }
        }
    }

    if ori == "v" {
        for i in 0..ship.size - 1 {
            /* input < 10 --> no field above */
            if player.own_board[input] != types::SubField::Water
                || input < 10 && i != ship.size - 1 {
                    return Err("Invalid position for this ship, please choose another coordinate."
                        .to_string())
            }
            input -= 10;
            indices.push(input);
        }
    } else {
        for i in 0..ship.size - 1 {
            /* unit position == 9 --> no field to the right */
            if player.own_board[input] != types::SubField::Water
                || (input % 10) == 9 && i != ship.size - 1 {
                    return Err("Invalid position for this ship, please choose another coordinate."
                        .to_string())
            }
            input += 1;
            indices.push(input);
        }
    }

    /* Places the ships on the board. */
    for i in indices.iter() {
        player.own_board[*i] = types::SubField::Ship;
    }

    Ok(())
}

/// Handles the initial ship placement for each player.
fn place_ships(mut player1: &mut types::Player, mut player2: &mut types::Player) {

    /* A vector of all the ships each player needs to place.
       The default version: #  Class of ship Size
                            4   Submarine     2
                            3   Destroyer     3
                            2   Cruiser       4
                            1   Battleship    5
    */
    let s1 = types::ShipType{ name: "Submarine".to_string(), size: 2, amount: 4 };
    let s2 = types::ShipType{ name: "Destroyer".to_string(), size: 3, amount: 3 };
    let s3 = types::ShipType{ name: "Cruiser".to_string(), size: 4, amount: 2 };
    let s4 = types::ShipType{ name: "Battleship".to_string(), size: 5, amount: 1 };
    let ships = vec![s1, s2, s3, s4];

    print_boards(&player1.own_board, &player1.op_board);

    /* Asks player1 to place the ships. */
    for i in ships.iter() {
        for _ in 0..i.amount {
            loop {
                println!("{}, please enter the first coordinate for your {:?} ({} fields).",
                    player1.name, i.name, i.size);
                match place(&mut player1, i) {
                    Ok(_) => { break; },
                    Err(e) => { println!("{}", e); },
                }
            }
            player1.capacity += i.size;
            print_boards(&player1.own_board, &player1.op_board);
        }
    }

    /* Asks player2 to place the ships. */
    for i in ships.iter() {
        for _ in 0..i.amount {
            loop {
                println!("{}, please enter the first coordinate for your {:?} ({} fields).",
                    player2.name, i.name, i.size);
                match place(&mut player2, i) {
                    Ok(_) => { break; },
                    Err(e) => { println!("{}", e); },
                }
            }
            player2.capacity += i.size;
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
        input = match helper::read_string().as_ref() {
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

/// Determines the type of the SubField that got hit
/// by the current move and sets it accordingly.
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

/// Initializes the players and the boards and provides the
/// game loop which lets the players perform their moves alternately.
pub fn start_round() {

    /* Creates the initial (empty) boards (10 x 10) for player1. */
    let mut player1 = types::Player {
        own_board: vec![types::SubField::Water; 100],
        op_board: vec![types::SubField::Water; 100],
        capacity: 0,
        name: "Player1".to_string(),
    };
    /* Creates the initial (empty) boards (10 x 10) for player2. */
    let mut player2 = types::Player {
        own_board: vec![types::SubField::Water; 100],
        op_board: vec![types::SubField::Water; 100],
        capacity: 0,
        name: "Player2".to_string(),
    };

    /* Initializes the boards with the player's ships. */
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
