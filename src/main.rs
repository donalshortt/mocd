mod parser;

fn main() {
    let mut game_data = mocb::Game::default();

    parser::parse(&mut game_data);   

    println!("waow");
}
