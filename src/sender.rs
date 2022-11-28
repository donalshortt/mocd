use reqwest::header::CONTENT_TYPE;

use serde_json::{json, Value};

fn to_json(game_data: &mocb::Game) -> Value {
    let json = json!({
        "date": game_data.date,
        "id": game_data.id,
        "players": game_data.players
    });

    json
}

pub fn send(game_data: &mocb::Game) {
    let json = to_json(&game_data);

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://4813212d-6429-430e-8f38-6a37061b64c7.mock.pstmn.io/hellothere/")
        .header(CONTENT_TYPE, "application/json")
        .json(&json)
        .send();                                
}
