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
