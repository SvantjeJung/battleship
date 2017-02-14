use term_painter::ToStyle;
use term_painter::Color::*;
use rand::{thread_rng, Rng};

pub mod types;
pub mod helper;

// Game logic (board, initialization, valid move, set, play or finished)

/// Visualization of the boards.
fn print_boards(board1: &[[types::SubField; 10]; 10], board2: &[[types::SubField; 10]; 10]) {
    println!("{}{}", "------------------ O W N   B O A R D ------------------",
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

    // The complete Moore neighborhood needs to be free
    // to place the first part of the ship.
    loop {
        // If every possibility was tested, we have a situation
        // in which the remaining boats can't be placed, so we
        // need to restart the whole placement process.
        if vec.len() == 1 {
            return Err(types::ErrorType::DeadEnd);
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

/// The actual placement of the ships.
fn place(player: &mut types::Player, ship: &types::ShipType) -> Result<(), types::ErrorType> {

    // Vector to collect the indices for the possible ship position.
    let mut indices = Vec::new();

    let mut input = get_input();
    loop {
        // The complete Moore neighborhood needs to be free
        // to place the first part of the ship.
        if valid_field(&player, input, "") {
            indices.push(input);
            break;
        } else {
            println!("Invalid input, again please.");
            input = get_input();
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

            // If the result would be negative, checked_sub()
            // returns None and input is set to 100 by unwrap_or().
            // In this case, it has to be the last part of the ship
            // because there's no field above.
            input = input.checked_sub(10).unwrap_or(100);

            if input == 100 && i != ship.size - 1 || !valid_field(&player, input, &ori) {
                return Err(types::ErrorType::InvalidField)
            }
            indices.push(input);
        }
    } else {
        for i in 0..ship.size - 1 {
            input += 1;
            // unit position == 9 --> no field to the right
            if (input % 10) == 0 && i != ship.size - 1 || !valid_field(&player, input, &ori) {
                return Err(types::ErrorType::InvalidField)
            }
            indices.push(input);
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
fn restart_placement(mut p1: &mut types::Player, mut p2: &mut types::Player, p1_call: bool) {
    if p1_call {
        p1.capacity = 0;
        for i in 0..10 {
            for j in 0..10 {
                p1.own_board[i][j] = types::SubField::Water;
            }
        }
    } else {
        p2.capacity = 0;
        for i in 0..10 {
            for j in 0..10 {
                p2.own_board[i][j] = types::SubField::Water;
            }
        }
    }
}

/// Handles the initial ship placement for each player.
fn place_ships(mut p1: &mut types::Player, mut p2: &mut types::Player)
    -> Result<(), types::ErrorType> {

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
    if p1.capacity == 0 {

        print_boards(&p1.own_board, &p1.op_board);

        // Asks player1 to place the ships.
        for i in ships.iter() {
            for _ in 0..i.amount {
                loop {
                    println!("{}, please enter the first coordinate for your {:?} ({}{}",
                        p1.name, i.name, i.size, " fields).");
                    match place(&mut p1, i) {
                        Ok(_) => { break; },
                        Err(e) => {
                            match e {
                                types::ErrorType::InvalidField => {
                                    println!("Invalid position for this ship, {}",
                                        "please choose another coordinate.");
                                },
                                types::ErrorType::DeadEnd => {},
                            }
                        },
                    }
                }
                p1.capacity += i.size;
                print_boards(&p1.own_board, &p1.op_board);
            }
        }
    }

    if p2.capacity == 0 {

        // Holds the remaining indices to place a ship at.
        let mut vec = Vec::new();
        for i in 0..100 {
            vec.push(i);
        }

        // Asks player2 to place the ships.
        for i in ships.iter() {
            for _ in 0..i.amount {
                if p2.player_type == types::PlayerType::Human {
                    print_boards(&p2.own_board, &p2.op_board);
                    loop {
                        println!("{}, please enter the first coordinate for your {:?} ({}{}",
                            p2.name, i.name, i.size, " fields).");
                        match place(&mut p2, i) {
                            Ok(_) => { break; },
                            Err(e) => {
                                match e {
                                    types::ErrorType::InvalidField => {
                                        println!("Invalid position for this ship, {}",
                                            "please choose another coordinate.");
                                    },
                                    types::ErrorType::DeadEnd => {},
                                }
                            },
                        }
                    }
                    print_boards(&p2.own_board, &p2.op_board);
                    p2.capacity += i.size;
                } else {
                    loop {
                        match place_ai(&mut p2, i, &mut vec) {
                            Ok(_) => { break; },
                            Err(e) => {
                                match e {
                                    types::ErrorType::InvalidField => {},
                                    types::ErrorType::DeadEnd => { return Err(e) },
                                }
                            },
                        }
                    }
                    print_boards(&p2.own_board, &p2.op_board);
                    p2.capacity += i.size;
                }
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
fn match_move(
    first: &mut types::Player,
    second: &mut types::Player,
    idx: usize
) -> types::SubField {

    let row = idx / 10;
    let col = idx % 10;

    match second.own_board[row][col] {
        types::SubField::Water => {
            println!("Miss - try again.");
            first.op_board[row][col] = types::SubField::WaterHit;
            return types::SubField::WaterHit
        },
        types::SubField::Ship => {
            println!("Hit!");
            first.op_board[row][col] = types::SubField::Hit;
            second.own_board[row][col] = types::SubField::Hit;
            second.capacity -= 1;
            return types::SubField::Hit
        },
        types::SubField::Hit => {
            println!("Already hit.");
            return types::SubField::Hit
        },
        types::SubField::WaterHit => {
            println!("Miss - try again.");
            return types::SubField::WaterHit
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
fn make_move(mut first: &mut types::Player, mut second: &mut types::Player) {
    if first.player_type == types::PlayerType::Human {
        println!("Enter coordinates, {}:", first.name);
        let input = get_input();
        match_move(&mut first, &mut second, input);
    } else if first.player_type == types::PlayerType::SmartAI {
        smart_move(&mut first, &mut second);
    } else {
        rand_move(&mut first, &mut second);
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

    let mut p2_type = types::PlayerType::Human;
    if mode == types::Mode::Single {
        p2_type = types::PlayerType::SmartAI;
        // p2_type = types::Player::DumbAI;
    }

    // Creates the initial (empty) boards (10 x 10) for player2.
    let mut player2 = types::Player {
        own_board: [[types::SubField::Water; 10]; 10],
        op_board: [[types::SubField::Water; 10]; 10],
        capacity: 0,
        player_type : p2_type,
        name: "Player2".to_string(),
    };

    // Initializes the boards with the player's ships.
    let mut cnt = 0;
    loop {
        match place_ships(&mut player1, &mut player2) {
            Ok(_) => { break; },
            Err(types::ErrorType::DeadEnd) => {
                // DeadEnd --> AI restarts the placement.
                cnt += 1;
                println!("DeadEnd {}", cnt);
                restart_placement(&mut player1, &mut player2, false);
            },
            Err(_) => {},
        }
    }

    loop {
        print_boards(&player1.own_board, &player1.op_board);
        make_move(&mut player1, &mut player2);
        if game_over(&player2) {
            println!("G A M E   O V E R");
            println!("Congratulations, Player1");
            break;
        }

        if mode != types::Mode::Single {
            print_boards(&player2.own_board, &player2.op_board);
        } else {
            print_boards(&player2.own_board, &player2.op_board);
            println!("AI - Move:");
        }

        make_move(&mut player2, &mut player1);
        if game_over(&player1) {
            println!("G A M E   O V E R");
            println!("Congratulations, Player2");
            break;
        }
    }
}
