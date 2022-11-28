pub struct Player {
    pub igns: Vec<String>,
    pub tag: String,
    pub score: u32,
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
    pub date: String,
    pub id: String,
    pub players: Vec<Player>,
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
