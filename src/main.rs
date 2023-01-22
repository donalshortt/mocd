mod parser;
mod sender;

extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use std::env;
use std::fs;
use std::fs::*;
use std::{thread, time};
use std::path::Path;
use std::io::Read;

// every 20 seconds, check if the metadata last modified has changed
// if yes, do the roar,
// if no, wait another 20 seconds
// take into account that this function might exited and restarted

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game_data = mocp_lib::Game::default();
    let filepath = "/home/donal/projects/moc/mocp/saves/mp_autosave.eu4";
    //let mut last_metadata: time::SystemTime = time::SystemTime::now();
    let mut last_time: String = String::new();

    let mut file: File;
    if Path::new("last_metadata.txt").exists() {
        file = File::open("last_metadata.txt").expect("Unable to open file to read when metadata was last used");
    } else {
        file = File::create("last_metadata.txt").unwrap();
    }
    file.read_to_string(&mut last_time).unwrap();
    

    loop {
        let latest_metadata = fs::metadata(filepath).unwrap().modified().unwrap();
        let latest_datetime: DateTime<Utc> = latest_metadata.into();
        let latest_time = latest_datetime.format("%T").to_string();

        println!("{}", latest_time);
        println!("{}", last_time);
        if latest_time != last_time {
            // write to file instead of this variable

            fs::write("last_metadata.txt", latest_time).expect("Unable to write time to file!");

            parser::parse(filepath, &mut game_data);
            sender::send(&game_data);

            println!("Sent!");
        } else {
            println!("Sleeping....");
            thread::sleep(time::Duration::new(5, 0));
        }
    }
}
