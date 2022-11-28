mod parser;

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game_data = mocb::Game::default();

    parser::parse(&mut game_data);   

    println!("waow");
}
