mod parser;
mod sender;

use std::env;
use std::fs;
use std::{thread, time};

// every 20 seconds, check if the metadata last modified has changed
// if yes, do the roar,
// if no, wait another 20 seconds
// take into account that this function might exited and restarted

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game_data = mocb::Game::default();
    let filepath = "/home/donal/projects/moc/mocp/saves/mp_autosave.eu4";
    let mut last_metadata: time::SystemTime = time::SystemTime::now();

    loop {
        let latest_metadata = fs::metadata(filepath).unwrap().modified().unwrap();
        
        if latest_metadata != last_metadata {
            last_metadata = latest_metadata;

            parser::parse(filepath, &mut game_data);
            sender::send(&game_data);

            println!("Parsed and sent!");
        } else {
            println!("Sleeping....");
            thread::sleep(time::Duration::new(5, 0));
        }
    }
}
