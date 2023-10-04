use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Value};

use crate::Game;

// TODO: derive serialize?
fn to_json(game_data: &Game) -> Value {
	let json = json!({
		"date": game_data.parsed_game.date,
		"name": game_data.name,
		"id": game_data.id,
		"players": game_data.parsed_game.players
	});

	json
}

pub fn send(game_data: &Game) {
	let json = to_json(&game_data);

	let client = reqwest::blocking::Client::new();
	let _res = client
		.post("http://10.15.10.193:3080/api/game_data")
		.header(CONTENT_TYPE, "application/json")
		.json(&json)
		.send();
}
