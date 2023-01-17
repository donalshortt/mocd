mod parser;
mod sender;

use std::env;
use std::fs;
use std::{thread, time};

// every 20 seconds, check if the metadata last modified has changed
// if yes, do the roar,
// if no, wait another 20 seconds

fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game_data = mocb::Game::default();
    let filepath = "/home/donal/projects/moc/mocp/saves/mp_autosave.eu4";
    let last_metadata;

    loop {
        if (latest_metadata != last_metadata) {
            let latest_metadata = fs::metadata(filepath)?;

            parser::parse(filepath, &mut game_data);
            sender::send(&game_data);
        } else {
            thread::sleep(10000);
        }
    }

    Ok(())
}
