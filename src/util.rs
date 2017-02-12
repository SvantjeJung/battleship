use model::types::{Board, SubField};

/// Reads a string from the terminal/user.
pub fn read_string() -> String {
    use std::io::stdin;
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Reads a valid `usize` integer from the terminal/user.
pub fn read_usize() -> usize {
    loop {
        match read_string().parse::<usize>() {
            Ok(res) => return res,
            Err(_) => println!("That was not an unsigned integer! Please try again!"),
        }
    }
}

/// Reads external board configuration
/// Does not check for valid ship placement!
pub fn read_extern_board(f: &str) -> Vec<SubField> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    let file;

    match File::open(f) {
        Ok(f) => file = f,
        Err(_) => return Board::init()
    }

    let input = BufReader::new(&file);
    let mut board = Board::init();
    let mut id = 0;
    for line in input.lines() {
        let l = line.unwrap();
        if l.starts_with("#") {
            continue
        }
        for c in l.chars() {
            match c {
                'X' => {
                    board[id] = SubField::Ship;
                    id += 1
                },
                '-' => id += 1,
                _ => {}
            }
        }
    }

    if id != 99 {
        Board::init();
    }

    board
}
