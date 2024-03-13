use std::{
	fs::{self, File, OpenOptions},
	io::Read,
	path::Path,
};

use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameListing {
	pub(crate) name: String,
	pub(crate) time_created: String,
	pub(crate) last_updated: String,
	pub(crate) uuid: String,
}

pub fn check_exists() {
	if Path::new("db.json").exists() {
		return;
	}

	let empty_vec: Vec<u8> = Vec::new();
	let empty_json_array = serde_json::to_string(&empty_vec).expect("failed to serialize");

	File::create("db.json").expect("failed to create db file");
	fs::write("db.json", empty_json_array).expect("failed to write empty vec");
}

pub fn read_listings() -> Vec<GameListing> {
	let mut file_content = String::new();

	File::open("db.json")
		.expect("failed to open db file")
		.read_to_string(&mut file_content)
		.expect("failed to read file to var");

	let listings: Vec<GameListing> =
		serde_json::from_str(&file_content).expect("failed to deserialize");

	listings
}

pub fn read_games() -> Vec<String> {
	let listings: Vec<GameListing> = read_listings();
	let mut games: Vec<String> = Vec::new();

	for listing in listings {
		games.push(listing.name);
	}

	games
}

pub fn write_listings(listings: Vec<GameListing>) {
	let file_path = "db.json";
	let mut file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(file_path)
		.expect("failed to open db");

	serde_json::to_writer_pretty(&mut file, &listings).expect("failed to write to db");
}

pub fn create_gamelisting(name: String) {
	let current_date = Local::now().date_naive();

	let listing = GameListing {
		name,
		time_created: current_date.to_string(),
		last_updated: current_date.to_string(),
		uuid: Uuid::new_v4().to_string(),
	};

	//TODO: is it really necessary to read and write here?
	let mut listings = read_listings();
	listings.push(listing);
	write_listings(listings);
}
