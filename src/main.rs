mod db;
mod parser;
mod sender;
mod ui;

extern crate chrono;

use crate::ui::StatefulList;
use chrono::offset::Utc;
use chrono::{DateTime, Local};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use ui::GameListing;
use std::error::Error;
use std::fs;
use std::fs::*;
use std::io;
use std::io::Read;
use std::path::Path;

use uuid::Uuid;

use tui::{backend::CrosstermBackend, Terminal};

enum AppState {
	Dashboard,
	GameSelect,
	NewGame,
}

pub struct App<'a> {
	app_state: AppState,
	games: StatefulList<'a>,
}

impl Default for App<'_> {
	fn default() -> Self {
		App {
			app_state: AppState::GameSelect,
			games: StatefulList::default(),
		}
	}
}

pub struct Game {
	pub date: String,
	pub name: String,
	pub id: String,
	pub players: Vec<Player>,
}

impl Default for Game {
	fn default() -> Game {
		Game {
			date: String::new(),
			name: String::new(),
			id: String::new(),
			players: Vec::new(),
		}
	}
}

pub struct Player {
	pub igns: Vec<String>,
	pub tag: String,
	pub score: u32,
}

impl Default for Player {
	fn default() -> Player {
		Player {
			igns: Vec::new(),
			tag: String::new(),
			score: 0,
		}
	}
}

impl Serialize for Player {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("player", 3)?;
		state.serialize_field("igns", &self.igns)?;
		state.serialize_field("tag", &self.tag[1..&self.tag.len() - 1])?;
		state.serialize_field("score", &self.score)?;
		state.end()
	}
}

// press "c" to create a new game
// -> open up the new game window
// -> enter game name
// -> generate the time created
// -> set time updated equal to time created
// -> generate uuid for the gamelisting
// -> maybe the game name should be it's identifier?

fn create_gamelisting(name: String) {
    let current_date = Local::now().date_naive();

    let listing = GameListing {
        name,
        time_created: current_date.to_string(),
        last_updated: current_date.to_string(),
        uuid: Uuid::new_v4().to_string(),       
    };
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
	db::check_exists();
	let mut app = App::default();
    let mut user_input = String::new();

	loop {
		match app.app_state {
			AppState::GameSelect => {
				ui::gameselect(terminal, &mut app)
					.expect("unable to display game selection screen");

				if let Event::Key(key) = event::read()? {
					match key.code {
						KeyCode::Char('q') => return Ok(()),
						KeyCode::Char('c') => app.app_state = AppState::NewGame,
						KeyCode::Down => app.games.next(),
						KeyCode::Up => app.games.previous(),
						_ => {}
					}
				}
			}

			AppState::Dashboard => {
				// ui::dashboard();
				println!("dashboard");
			}

			AppState::NewGame => {
				if let Event::Key(key) = event::read()? {
					match key.code {
						KeyCode::Enter => {
							create_gamelisting(user_input.drain(..).collect());
						}
						KeyCode::Char(c) => {
                            user_input.push(c);
                        }
						KeyCode::Backspace => {
                            user_input.pop(); 
                        }
						KeyCode::Esc => { 
                            app.app_state = AppState::GameSelect
                        }
						_ => {}
					}
				}
			}
		}

		/*let latest_metadata = fs::metadata(filepath)
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
		}*/
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut terminal = ui::ui_setup().unwrap();

	//ui::update_dashboard(ui);
	run_app(&mut terminal).expect("app failed to start");

	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	terminal.show_cursor()?;

	Ok(())
	/*
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

		loop {
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
		}
	*/
}
