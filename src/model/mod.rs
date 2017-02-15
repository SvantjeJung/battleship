use util;
use term_painter::ToStyle;
use term_painter::Color::*;
use rand::{thread_rng, Rng};

pub mod types;

// Game logic (board, initialization, valid move, set, play or finished)

/// Visualization of the boards.
fn print(board1: &[[types::SubField; 10]; 10], board2: &[[types::SubField; 10]; 10]) {
    println!("\n{}{}", "------------------ O W N   B O A R D ------------------",
        "-------------------------- O P P O N E N T ---------------");

    let mut cnt = 9;

    for row in 0..10 {
        print!("\n{0:<width$}", cnt, width=3);
        for field in 0..10 {
            if board1[row][field] == types::SubField::Hit {
                print!(" {} ", Red.paint(board1[row][field]));
            } else {
                print!(" {} ", board1[row][field]);
            }
        }
        print!("        ");
        print!("{}  ", cnt);
        for field in 0..10 {
            if board2[row][field] == types::SubField::Hit {
                print!(" {} ", Green.paint(board2[row][field]));
            } else {
                print!(" {} ", board2[row][field]);
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
    println!("");
}

/// Print boards of player
/// Encapsulates print_boards(board1, board2)
pub fn print_boards(player: &types::Player) {
    print(&player.own_board, &player.op_board);
}

/// Determines whether the chosen field is a valid one. Considers the
/// Moore neighborhood because the ships shouldn't collide.
fn valid_field(player: &types::Player, input: usize, ori: &str) -> bool {

    let mut valid = false;

    let row = input / 10;
    let col = input % 10;

    match ori {
        "v" => {
            // The last part of the ship - top-left corner.
            if input == 0 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The last part of the ship - top-right corner.
            } else if input == 9 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The last part of the ship - top row (except the corners).
            } else if input < 10 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The left column except the corners.
            } else if input % 10 == 0 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col + 1] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The right column except the corners.
            } else if input % 10 == 9 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The ordinary rest of the fields. A downwards collision is
            // allowed since it is the same ship.
            } else {
                valid = check_ordinary(&player, row, col, ori);
            }
        },
        "h" => {
            // The last part of the ship - top-right corner.
            if input == 9 {
                if player.own_board[row][col] == types::SubField::Water
                   && player.own_board[row + 1][col] == types::SubField::Water
                   && player.own_board[row + 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The last part of the ship - bottom-right corner.
            } else if input == 99 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The bottom row except the corners.
            } else if input / 10 == 9 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                    && player.own_board[row - 1][col + 1 ] == types::SubField::Water
                {
                    valid = true;
                }
            // The top row except the corners.
            } else if input < 10 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The last part of the ship - right column except the corners.
            } else if input % 10 == 9 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The ordinary rest of the fields. The left-hand side is allowed
            // to be occupied since it is the same ship.
            } else {
                valid = check_ordinary(&player, row, col, ori);
            }
        },
        _ => {
            // The first part of the ship needs to be fully surrounded by water.
            // Checks the Moore neighborhood.
            // The top-left corner.
            if input == 0 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The top-right corner.
            } else if input == 9 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The bottom-right corner.
            } else if input == 99 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The bottom-left corner.
            } else if input == 90 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The bottom row except the corners.
            } else if input / 10 == 9 {
               if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                    && player.own_board[row - 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The left column except the corners.
            } else if input % 10 == 0 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                    && player.own_board[row - 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The right column except the corners.
            } else if input % 10 == 9 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row - 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row - 1][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The top row except the corners.
            } else if input < 10 {
                if player.own_board[row][col] == types::SubField::Water
                    && player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row][col + 1] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                    && player.own_board[row + 1][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col + 1] == types::SubField::Water
                {
                    valid = true;
                }
            // The ordinary rest of the fields.
            } else {
                valid = check_ordinary(&player, row, col, ori);
            }
        },
    }
    valid
}

/// Checks the neighborhood of the "ordinary" fields depending on the ship's orientation.
/// If it is the first part of the ship, the whole moore-neighborhood gets checked.
/// Otherwise the left-hand respectively bottom field is ignored.
fn check_ordinary(player: &types::Player, row: usize, col: usize, ori: &str) -> bool {

    if player.own_board[row][col] == types::SubField::Water
        && player.own_board[row][col + 1] == types::SubField::Water
        && player.own_board[row - 1][col] == types::SubField::Water
        && player.own_board[row - 1][col - 1] == types::SubField::Water
        && player.own_board[row - 1][col + 1] == types::SubField::Water
        && player.own_board[row + 1][col - 1] == types::SubField::Water
        && player.own_board[row + 1][col + 1] == types::SubField::Water
    {
        match ori {
            "v" => {
                if player.own_board[row][col - 1] == types::SubField::Water {
                    return true;
                }
            },
            "h" => {
                if player.own_board[row + 1][col] == types::SubField::Water {
                    return true;
                }
            },
            _ => {
                if player.own_board[row][col - 1] == types::SubField::Water
                    && player.own_board[row + 1][col] == types::SubField::Water
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Removes the current rand value from the remaining possibilities.
/// Since every call of remove() shifts every remaining element to the left,
/// we need to search for the index of the element to be deleted.
fn remove_idx(rand: usize, vec: &mut Vec<usize>) {
    for i in 0..vec.len() - 1 {
        if vec[i] == rand {
            vec.remove(i);
        }
    }
}

/// Random ship placement for the AI.
fn place_ai(
    player: &mut types::Player,
    ship: &types::ShipType,
    mut vec: &mut Vec<usize>
) -> Result<(), types::ErrorType> {

    // Vector to collect the indices for the possible ship position.
    let mut indices = Vec::new();

    let mut rng = thread_rng();
    let mut rand = *rng.choose(&vec).unwrap();

    // The complete Moore-neighborhood needs to be free
    // to place the first part of the ship.
    loop {
        // If every possibility was tested, we have a situation
        // in which the remaining boats can't be placed, so we
        // need to restart the whole placement process.
        if vec.len() == 1 {
            return Err(types::ErrorType::DeadEndPlayer2);
        } else if valid_field(&player, rand, "") {
            indices.push(rand);
            remove_idx(rand, &mut vec);
            break;
        // Invalid fields should be removed from vec.
        } else {
            remove_idx(rand, &mut vec);
            rand = *rng.choose(&vec).unwrap();
        }
    }

    // Random bool to determine the orientation.
    let ori = rng.gen::<bool>();

    // Vertical
    if ori {
        for i in 0..ship.size - 1 {

            // If the result would be negative, checked_sub()
            // returns None and input is set to 100 by unwrap_or().
            // In this case, it has to be the last part of the ship
            // because there's no field above.
            rand = rand.checked_sub(10).unwrap_or(100);

            if rand == 100 && i != ship.size - 1 || !valid_field(&player, rand, "v") {
                remove_idx(rand, &mut vec);
                return Err(types::ErrorType::InvalidField)
            }
            indices.push(rand);
        }
    } else {
        for i in 0..ship.size - 1 {
            rand += 1;
            // unit position == 9 --> no field to the right
            if (rand % 10) == 0 && i != ship.size - 1 || !valid_field(&player, rand, "h") {
                remove_idx(rand, &mut vec);
                return Err(types::ErrorType::InvalidField)
            }
            indices.push(rand);
        }
    }

    // Places the ships on the board.
    for i in indices.iter() {
        let row = *i / 10;
        let col = *i % 10;
        player.own_board[row][col] = types::SubField::Ship;
    }

    Ok(())
}

/// Checks whether there is a remaining position to place the current ship at.
fn available_space(player: &types::Player, ship: &types::ShipType) -> bool {

    let mut v_cnt = 1;
    let mut h_cnt = 1;

    for i in 0..100 {

        let mut v_val = i;
        let mut h_val = i;

        if valid_field(&player, i, "") {

            for j in 0..ship.size - 1 {

                v_val = v_val.checked_sub(10).unwrap_or(100);
                if v_val == 100 && j != ship.size - 1 || !valid_field(&player, v_val, "v") {
                } else { v_cnt += 1; }

                h_val += 1;
                if (h_val % 10) == 0 && j != ship.size - 1 || !valid_field(&player, h_val, "h") {
                } else { h_cnt += 1; }
            }
        }

        if v_cnt == ship.size || h_cnt == ship.size {
            return true
        } else {
            v_cnt = 1;
            h_cnt = 1;
        }
    }
    false
}

/// The actual placement of the ships.
fn place(player: &mut types::Player, ship: &types::ShipType) -> Result<(), types::ErrorType> {

    if player.name == "Player1" {
        if !available_space(&player, &ship) {
           return Err(types::ErrorType::DeadEndPlayer1)
        }    
    } else {
        if !available_space(&player, &ship) {
            return Err(types::ErrorType::DeadEndPlayer2)
        }
    }

    // Vector to collect the indices for the possible ship position.
    let mut indices = Vec::new();

    let input = util::read_string();
    let mut idx = types::Board::get_index(&input);
    loop {
        while idx == 100 {
            println!("Invalid input, again please.");
            idx = types::Board::get_index(&util::read_string());
        }
        // The complete Moore neighborhood needs to be free
        // to place the first part of the ship.
        if valid_field(&player, idx, "") {
            indices.push(idx);
            break;
        } else {
            println!("Invalid input, again please.");
            idx = types::Board::get_index(&util::read_string());
        }
    }

    println!(
        "Enter 'h' for a horizontal (rightwards) orientation of the ship,
        'v' for a vertical (upwards) one."
    );

    let mut ori = util::read_string();

    loop {
        match ori.as_str() {
            "h" | "v" => break,
            _ => {
                println!("Invalid input, again please.");
                ori = util::read_string();
            }
        }
    }

    if ori == "v" {
        for i in 0..ship.size - 1 {

            // If the result would be negative, checked_sub()
            // returns None and input is set to 100 by unwrap_or().
            // In this case, it has to be the last part of the ship
            // because there's no field above.
            idx = idx.checked_sub(10).unwrap_or(100);

            if idx == 100 && i != ship.size - 1 || !valid_field(&player, idx, &ori) {
                return Err(types::ErrorType::InvalidField)
            }
            indices.push(idx);
        }
    } else {
        for i in 0..ship.size - 1 {
            idx += 1;
            if (idx % 10) == 0 && i != ship.size - 1 || !valid_field(&player, idx, &ori) {
                return Err(types::ErrorType::InvalidField)
            }
            indices.push(idx);
        }
    }

    // Places the ships on the board.
    for i in indices.iter() {
        let row = *i / 10;
        let col = *i % 10;
        player.own_board[row][col] = types::SubField::Ship;
    }

    Ok(())
}

/// Resets the particular player's board to prepare the (re)placement.
fn restart_placement(p: &mut types::Player) {
    p.capacity = 0;
    for i in 0..10 {
        for j in 0..10 {
            p.own_board[i][j] = types::SubField::Water;
        }
    }
}

/// Handles the initial ship placement for each player.
pub fn place_ships(mut p: &mut types::Player) -> Result<(), types::ErrorType> {

    // A vector of all the ships each player needs to place.
    // The default version: #  Class of ship Size
    //                      4   Submarine     2
    //                      3   Destroyer     3
    //                      2   Cruiser       4
    //                      1   Battleship    5
    //
    let s1 = types::ShipType{ name: "Submarine".to_string(), size: 2, amount: 4 };
    let s2 = types::ShipType{ name: "Destroyer".to_string(), size: 3, amount: 3 };
    let s3 = types::ShipType{ name: "Cruiser".to_string(), size: 4, amount: 2 };
    let s4 = types::ShipType{ name: "Battleship".to_string(), size: 5, amount: 1 };
    let ships = vec![s1, s2, s3, s4];

    // In a replacement situation for p2,
    // p1 doesn't need to place the ships again.
    if p.capacity == 0 && p.name == "Player1" {

        print_boards(&p);

        // Asks player1 to place the ships.
        for i in ships.iter() {
            for _ in 0..i.amount {
                loop {
                    println!("{}, please enter the first coordinate for your {:?} ({}{}",
                        p.name, i.name, i.size, " fields).");
                    match place(&mut p, i) {
                        Ok(_) => { break; },
                        Err(e) => {
                            match e {
                                types::ErrorType::InvalidField => {
                                    println!("Invalid position for this ship, {}",
                                        "please choose another coordinate.");
                                },
                                types::ErrorType::DeadEndPlayer1 => { return Err(e) },
                                _ => { return Err(e) },
                            }
                        },
                    }
                }
                p.capacity += i.size;
                print_boards(&p);
            }
        }
    }

    if p.capacity == 0 && p.name == "Player2" {

        // Holds the remaining indices to place a ship at.
        let mut vec = Vec::new();
        for i in 0..100 {
            vec.push(i);
        }

        // Asks player2 to place the ships.
        for i in ships.iter() {
            for _ in 0..i.amount {
                loop {
                    match place_ai(&mut p, i, &mut vec) {
                        Ok(_) => { break; },
                        Err(e) => {
                            match e {
                                types::ErrorType::InvalidField => {},
                                types::ErrorType::DeadEndPlayer2 => { return Err(e) },
                                _ => { return Err(e) },
                            }
                        },
                    }
                }
                print_boards(&p);
                p.capacity += i.size;
            }
        }
    }
    Ok(())
}

/// Reads the coordinates of a field from the user
/// and returns the corresponding index.
fn get_input() -> usize {

    // 100 --> invalid
    let mut input = 100;

    while input == 100 {
        input = match util::read_string().as_ref() {
            "A0" => 90, "A1" => 80, "A2" => 70, "A3" => 60, "A4" => 50,
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

/// Check if given input is a valid coordinate
pub fn valid_coordinate(input: &str) -> bool {
    if input.len() != 2 {
        return false
    }

    let lower = input.to_owned().to_lowercase();
    let mut alpha = false;
    let mut num = false;

    for c in lower.chars() {
        if !alpha && !num {
            if valid_alpha(c) {
                alpha = true;
            } else if valid_num(c) {
                num = true;
            } else {
                return false;
            }
        } else if !alpha {
            alpha = valid_alpha(c);
        } else if !num {
            num = valid_num(c);
        } else {
            return false;
        }
    }

    alpha && num
}

/// Check if given character is of valid alphabetic value
fn valid_alpha(c: char) -> bool {
    match c {
        'a' ... 'j' => true,
        _ => false,
    }
}

/// Check if given character is of valid numeric value
fn valid_num(c: char) -> bool {
    match c {
        '0' ... '9' => true,
        _ => false,
    }
}

/// Determines the type of the SubField that got hit
/// by the current move and sets it accordingly.
pub fn match_move(
    attacker: &mut types::Player,
    opponent: &mut types::Player,
    idx: usize
) -> types::SubField {

    let row = idx / 10;
    let col = idx % 10;

    match opponent.own_board[row][col] {
        types::SubField::Water => {
            println!("Miss - try again.");
            attacker.op_board[row][col] = types::SubField::Miss;
            return types::SubField::Miss
        },
        types::SubField::Ship => {
            println!("Hit!");
            attacker.op_board[row][col] = types::SubField::Hit;
            opponent.own_board[row][col] = types::SubField::Hit;
            opponent.capacity -= 1;
            return types::SubField::Hit
        },
        types::SubField::Hit => {
            println!("Already hit.");
            return types::SubField::Hit
        },
        types::SubField::Miss => {
            println!("Miss - try again.");
            return types::SubField::Miss
        }
    }
}

/// Function which provides random moves for the "dumb" ai.
fn rand_move(mut first: &mut types::Player, mut second: &mut types::Player) {
    let mut rng = thread_rng();
    let rand = rng.gen_range(0, 100);
    match_move(&mut first, &mut second, rand);
}

/// Calculates smart moves for the ai.
fn smart_move(mut first: &mut types::Player, mut second: &mut types::Player) {

    // Holds the remaining indices to aim on.
    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }

    let mut target = 100;

    // Checks the surrounding of a hit.
    for row in 0..10 {
        for col in 0..10 {
            if first.op_board[row][col] == types::SubField::Hit {
                // The special cases:
                if row == 0 && col == 0 {
                    if first.op_board[row][col + 1] == types::SubField::Water {
                        target = 1;
                    } else {
                        if first.op_board[row + 1][col] == types::SubField::Water {
                            target = 10;
                        }
                    }
                } else if row == 0 {
                    if col == 9 {
                        if first.op_board[row][col - 1] == types::SubField::Water {
                            target = col - 1;
                        } else {
                            if first.op_board[row + 1][col] == types::SubField::Water {
                                target = 10 + col;
                            }
                        }
                    } else if first.op_board[row][col + 1] == types::SubField::Water {
                        target = col + 1;
                    } else if first.op_board[row][col - 1] == types::SubField::Water {
                        target = col - 1;
                    } else {
                        if first.op_board[row + 1][col] == types::SubField::Water {
                            target = 10 + col;
                        }
                    }
                } else if col == 0 {
                    if row == 9 {
                        if first.op_board[row - 1][col] == types::SubField::Water {
                           target = (row - 1) * 10;
                        } else {
                            if first.op_board[row][col + 1] == types::SubField::Water {
                                target = (row * 10) + 1;
                            }
                        }
                    } else if first.op_board[row][col + 1] == types::SubField::Water {
                        target = row * 10 + 1;
                    } else if first.op_board[row - 1][col] == types::SubField::Water {
                        target = (row - 1) * 10;
                    } else {
                        if first.op_board[row + 1][col] == types::SubField::Water {
                            target = (row + 1) * 10;
                        }
                    }
                } else if row == 9 && col == 9 {
                    if first.op_board[row][col - 1] == types::SubField::Water {
                        target = row * 10 + col - 1;
                    } else {
                        if first.op_board[row - 1][col] == types::SubField::Water {
                            target = (row - 1) * 10 + col;
                        }
                    }
                } else if row == 9 {
                    if first.op_board[row][col - 1] == types::SubField::Water {
                        target = row * 10 + col - 1;
                    } else if first.op_board[row][col + 1] == types::SubField::Water {
                        target = row * 10 + col + 1;
                    } else {
                        if first.op_board[row - 1][col] == types::SubField::Water {
                            target = (row - 1) * 10 + col;
                        }
                    }
                } else if col == 9 {
                    if first.op_board[row][col - 1] == types::SubField::Water {
                        target = row * 10 + col - 1;
                    } else if first.op_board[row + 1][col] == types::SubField::Water {
                        target = (row + 1) * 10 + col;
                    } else {
                        if first.op_board[row - 1][col] == types::SubField::Water {
                            target = (row - 1) * 10 + col;
                        }
                    }
                // The ordinary cases.
                } else {
                    if first.op_board[row][col - 1] == types::SubField::Water {
                        target = row * 10 + col - 1;
                    } else if first.op_board[row][col + 1] == types::SubField::Water {
                        target = row * 10 + col + 1;
                    } else if first.op_board[row - 1][col] == types::SubField::Water {
                        target = (row - 1) * 10 + col;
                    } else {
                        if first.op_board[row + 1][col] == types::SubField::Water {
                            target = (row + 1) * 10 + col;
                        }
                    }
                }
            }
            if target != 100 {
                break;
            }
        }
        if target != 100 {
            break;
        }
    }

    let mut rng = thread_rng();

    if target == 100 {
        target = *rng.choose(&vec).unwrap();
    }

    // Calculates fields as long as an unused one occurs.
    loop {
        let row = target / 10;
        let col = target % 10;
        // It shouldn't hit a target twice.
        if first.op_board[row][col] == types::SubField::Water { break; }
        remove_idx(target, &mut vec);
        target = *rng.choose(&vec).unwrap();
    }
    match_move(&mut first, &mut second, target);
}

/// Lets the players perform their moves.
fn make_move(mut attacker: &mut types::Player, mut opponent: &mut types::Player) {
    if attacker.player_type == types::PlayerType::Human {
        println!("Enter coordinates, {}:", attacker.name);
        let input = get_input();
        match_move(&mut attacker, &mut opponent, input);
    } else if attacker.player_type == types::PlayerType::SmartAI {
        smart_move(&mut attacker, &mut opponent);
    } else {
        rand_move(&mut attacker, &mut opponent);
    }
}

/// Returns whether all the player's ships got destroyed.
pub fn game_over(player: &types::Player) -> bool {
    player.capacity <= 0
}

/// Initializes the players and the boards and provides the
/// game loop which lets the players perform their moves alternately.
pub fn start_round(mode: types::Mode) {

    // Creates the initial (empty) boards (10 x 10) for player1.
    let mut player1 = types::Player {
        own_board: [[types::SubField::Water; 10]; 10],
        op_board: [[types::SubField::Water; 10]; 10],
        capacity: 0,
        // Could be extended later to have an AI vs. AI version.
        player_type: types::PlayerType::Human,
        name: "Player1".to_string(),
    };

    // Creates the initial (empty) boards (10 x 10) for player2.
    let mut player2 = types::Player {
        own_board: [[types::SubField::Water; 10]; 10],
        op_board: [[types::SubField::Water; 10]; 10],
        capacity: 0,
        player_type : types::PlayerType::SmartAI,
        name: "Player2".to_string(),
    };

    // Initializes the boards with the player's ships.
    loop {
        match place_ships(&mut player1) {
            Ok(_) => { break; },
            Err(types::ErrorType::DeadEndPlayer1) => {
                println!("Human DeadEnd");
                restart_placement(&mut player1);
            },
            Err(_) => {},
        }
    }
    let mut cnt = 0;
    loop {
        match place_ships(&mut player2) {
            Ok(_) => { break; },
            Err(types::ErrorType::DeadEndPlayer2) => {
                cnt += 1;
                println!("AI DeadEnd {}", cnt);
                restart_placement(&mut player2);
            },
            Err(_) => {},
        }    
    }
    
    loop {
        print_boards(&player1);
        make_move(&mut player1, &mut player2);
        if game_over(&player2) {
            println!("G A M E   O V E R");
            println!("Congratulations, Player1");
            break;
        }

        print_boards(&player2);
        println!("AI - Move:");

        make_move(&mut player2, &mut player1);
        if game_over(&player1) {
            println!("G A M E   O V E R");
            println!("Congratulations, Player2");
            break;
        }
    }
}
