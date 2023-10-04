use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Value};

use crate::ParsedGame;

// TODO: derive serialize?
fn to_json(game_data: &ParsedGame) -> Value {
	let json = json!({
		"date": game_data.date,
		"name": game_data.name,
		"id": game_data.id,
		"players": game_data.players
	});

	json
}

pub fn send(game_data: &ParsedGame) {
	let json = to_json(&game_data);

	let client = reqwest::blocking::Client::new();
	let _res = client
		.post("http://10.15.10.193:3080/api/game_data")
		.header(CONTENT_TYPE, "application/json")
		.json(&json)
		.send();
}
