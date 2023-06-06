mod parser;
mod sender;
mod ui;

extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use std::fs;
use std::fs::*;
use std::io::Read;
use std::path::Path;
use std::{thread, time};

fn main() {
    let ui = ui::ui_setup().unwrap();

    ui::update_dashboard(ui);

    loop {}

	let mut game_data = mocp_lib::Game::default();
	let filepath = "/home/donal/projects/moc/mocp/saves/mp_autosave.eu4";
	let mut last_time: String = String::new();

	let mut file: File;
	if !Path::new("last_metadata.txt").exists() {
		File::create("last_metadata.txt")
			.expect("Could not create last file to record time of last metadata access");
	}

	file = File::open("last_metadata.txt")
		.expect("Unable to open file to read time of last metadata access");
	file.read_to_string(&mut last_time).unwrap();

	/*loop {
		let latest_metadata = fs::metadata(filepath)
			.expect("Couldn't get metadata from savefile")
			.modified()
			.expect("Couldn't get time modified from metadata");
		let latest_datetime: DateTime<Utc> = latest_metadata.into();
		let latest_time = latest_datetime.format("%T").to_string();

		println!("Latest time from metadata: {}", latest_time);
		println!("Last time from file: {}", last_time);
		if latest_time != last_time {
			// write to file instead of this variable
			last_time = latest_time.clone();
			fs::write("last_metadata.txt", last_time.clone())
				.expect("Unable to write time last modified to file");

			fs::write("last_metadata.txt", latest_time).expect("Unable to write time to file!");

			parser::parse(filepath, &mut game_data);
			sender::send(&game_data);

			println!("Sent!");
		} else {
			println!("Sleeping....");
			thread::sleep(time::Duration::new(5, 0));
		}
	}*/
}
