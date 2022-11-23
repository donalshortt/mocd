use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Enumerate;
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

fn player_exists(tag: &String, game_data: &Game) -> bool {
    for player in &game_data.players {
        if player.tag.eq(tag) {
            return true;
        }
    }

    false
}

fn insert_country_data(buffer: &Vec<String>, game_data: &mut Game) {
    // strips the whitespace from the raw input lines and groups each line in tags and igns
    // TODO: figure out why this can't be a oneliner in it's current form
    let stripped_buf = buffer.into_iter().map(|x| x.trim().to_string()).collect::<Vec<String>>();
    let chunked_buf = stripped_buf.chunks(2);

    for x in chunked_buf {
        if !player_exists(&x[1], &game_data) {
            game_data.players.push(
                Player { 
                    igns: vec![x[0].clone()], 
                    tag: x[1].clone().to_string(), 
                    score: 0 
                }
            );
        } else {
            for player in &mut game_data.players {
                if player.tag.eq(&x[1]) {
                    player.igns.push(x[0].clone());                   
                }
            }
        }
    }
}

fn insert_score_data(buffer: &Vec<String>, game_data: &mut Game) {
    let mut iter = buffer.iter();

    loop {
        for player in &game_data.players {
            if iter  {

            }
        }

        iter.next();
    }
}

fn main() {
    let filepath = "/home/donal/projects/mocd/saves/mp_autosave.eu4";
    let mut game_data = Game::default();

    let lines = read_lines(filepath).expect("lines extracted from file");
    
    let mut reading_player_countries = false;
    let mut player_countries_buf: Vec<String> = Vec::new();

    let mut reading_player_scores = false;
    let mut player_scores_buf: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(ip) = line {

            if ip.contains("campaign_id") {
                let id_start = ip.find('"').unwrap_or(0);                   
                game_data.id = ip[(id_start + 1)..(ip.len() - 1)].to_string();
            }

            if reading_player_countries {
                if ip.contains("}") && (ip.chars().count() == 1) {
                    reading_player_countries = false;
                    insert_country_data(&player_countries_buf, &mut game_data);
                    continue;
                }

                player_countries_buf.push(ip.clone());
            }                

            if ip.contains("players_countries") {
                reading_player_countries = true;
            }

            if reading_player_scores {
                if ip.contains("}") && (ip.chars().count() == 1) {
                    reading_player_scores = false;
                    insert_score_data(&player_scores_buf, &mut game_data);
                    continue;
                }

                player_scores_buf.push(ip.clone());
            }

            if ip.contains("score_statistics") {
                reading_player_scores = true;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
