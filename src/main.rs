// extern crate clap;
extern crate term_painter;

mod client;
mod model;
mod server;
mod view;

fn main() {
    println!("Welcome to a round of 'battleship'");
    model::start_match();
}
