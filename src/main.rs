use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Player {
    ign: String,
    tag: String,
    score: u32,
}

impl Default for Player {
    fn default () -> Player {
        Player {
            ign : String::new(),
            tag : String::new(),
            score : 0,
        }
    }
}

struct Game {
    id: String,
    players: Vec<Player>,
}

impl Default for Game {
    fn default () -> Game {
        Game {
            id : String::new(),
            players : Vec::new(),
        }
    }
}

fn main() {
    let mut game_data = Game::default();

    if let Ok(lines) = read_lines("/home/donal/projects/mocd/saves/mp_autosave.eu4") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("campaign_id") {
                    let id_start = ip.find('"').unwrap_or(0);
                    
                    game_data.id = ip[(id_start + 1)..(ip.len() - 1)].to_string();
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

