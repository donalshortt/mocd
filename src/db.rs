use std::{path::Path, fs::{File, self}};

use tui::widgets::ListItem;

pub fn check_exists() {
	if !Path::new("db.json").exists() {
        File::create("db.json")
            .expect("unable to create db file");
    }
}

pub fn read_games(db: &String) -> Vec<String> {
    return serde_json::from_str(db)
        .expect("could not read games from db");
}

pub fn write_game(db: &String) {
    db.
}
