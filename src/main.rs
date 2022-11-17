use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Player {
    igns: Vec<String>,
    tag: String,
    score: u32,
}

impl Default for Player {
    fn default () -> Player {
        Player {
            igns : Vec::new(),
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
    let filepath = "/home/donal/projects/mocd/saves/mp_autosave.eu4";
    let mut game_data = Game::default();

    let lines = read_lines(filepath).expect("lines extracted from file");
    
    let mut reading_player_countries = false;
    let mut player_countries_buf : Vec<String> = Vec::new();

    for line in lines {
        if let Ok(ip) = line {

            if ip.contains("campaign_id") {
                let id_start = ip.find('"').unwrap_or(0);                   
                game_data.id = ip[(id_start + 1)..(ip.len() - 1)].to_string();
            }

            if reading_player_countries {
                if ip.contains("}") && (ip.chars().count() == 1) {
                    reading_player_countries = false;
                    continue;
                }

                player_countries_buf.push(ip.clone());
            }                

            if ip.contains("players_countries") {
                reading_player_countries = true;
            }               
        }
    }

    println!("Yo");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

