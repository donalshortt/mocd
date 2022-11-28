use std::collections::HashMap;

pub fn send(_game_data : mocb::Game) {

    let mut map2 = HashMap::new();
    map2.insert("wee", "woo");
    map2.insert("pee", "poo");

    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", map2);

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://4813212d-6429-430e-8f38-6a37061b64c7.mock.pstmn.io/hellothere/")
        .json(&map)
        .send();
                                
}
