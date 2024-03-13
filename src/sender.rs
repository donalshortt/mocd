use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Value};
use std::fmt::Write;

use crate::Game;
use crate::App;

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

pub fn send(app: &App) {
	let json = to_json(app.current_game.as_ref().unwrap());
    let mut url: String = String::new();
    let port = 80;
    let endpoint = "/api/game_data";


    write!(&mut url, "http://{}:{}{}", app.webserver_ip, port, endpoint).expect("failed to create url for webserver");

	let client = reqwest::blocking::Client::new();
	let _res = client
		.post(url)
		.header(CONTENT_TYPE, "application/json")
		.json(&json)
		.send();
}
