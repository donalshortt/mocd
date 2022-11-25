mod parser;

pub struct Player {
    igns: Vec<String>,
    tag: String,
    score: u32,
}

impl Default for Player {
    fn default () -> Player {
        Player {
            igns: Vec::new(),
            tag: String::new(),
            score: 0,
        }
    }
}

pub struct Game {
    date: String,
    id: String,
    players: Vec<Player>,
}

impl Default for Game {
    fn default () -> Game {
        Game {
            date: String::new(),
            id: String::new(),
            players: Vec::new(),
        }
    }
}

fn main() {
    let mut game_data = Game::default();

    parser::parse(&game_data);   
}
