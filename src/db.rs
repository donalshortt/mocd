use std::{path::Path, fs::{File, self}, io::Read};

use crate::GameListing;

pub fn check_exists() {
	if Path::new("db.json").exists() {
        return;
    }

    let empty_vec: Vec<u8> = Vec::new();
    let empty_json_array = serde_json::to_string(&empty_vec)
        .expect("failed to serialize");

    File::create("db.json")
        .expect("failed to create db file");
    fs::write("db.json", empty_json_array)
        .expect("failed to write empty vec");
}

pub fn read_games() -> Vec<GameListing> {
    let mut file_content = String::new();
    
    File::open("db.json")
        .expect("failed to open db file")
        .read_to_string(&mut file_content)
        .expect("failed to read file to var");
    
    let listings : Vec<GameListing> = serde_json::from_str(&file_content)
        .expect("failed to deserialize");

    listings
}
