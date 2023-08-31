mod parser;
mod sender;
mod ui;

extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use crossterm::event::{self, Event, KeyCode};
use std::fs;
use std::fs::*;
use std::io;
use std::io::Read;
use std::path::Path;
use std::{thread, time};
use tui::widgets::{ListItem, ListState};

use tui::{backend::CrosstermBackend, Terminal};

struct StatefulList<'a> {
	state: ListState,
	items: Vec<ListItem<'a>>,
}

// add a way to detect keyboard strokes and go up and down a list with the selection

impl Default for StatefulList<'_> {
	fn default() -> Self {
		StatefulList {
			state: ListState::default(),
			//TODO: create a function to get the list items from a datafile
			items: vec![
				ListItem::new("test"),
				ListItem::new("bigtest"),
				ListItem::new("quit"),
			],
		}
	}
}

impl StatefulList<'_> {
	fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len() - 1 {
					0
				} else {
					i + 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len() - 1
				} else {
					i - 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}
}

enum AppState {
	GameSelect,
	Dashboard,
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

// display my stateful list via a config for gameselect

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
	let mut app = App::default();

	loop {
		match app.app_state {
			AppState::GameSelect => {
				ui::gameselect(terminal, &mut app);
			}

			AppState::Dashboard => {
				// ui::dashboard();
				println!("dashboard");
			}
		}

		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char('q') => return Ok(()),
				KeyCode::Down => app.games.next(),
				KeyCode::Up => app.games.previous(),
				_ => {}
			}
		}

		/*if let Event::Key(key) = event::read()? {
			match app.input_mode {
				InputMode::Normal => match key.code {
					KeyCode::Char('e') => {
						app.input_mode = InputMode::Editing;
					}
					KeyCode::Char('q') => {
						return Ok(());
					}
					_ => {}
				},
				InputMode::Editing => match key.code {
					KeyCode::Enter => {
						app.messages.push(app.input.drain(..).collect());
					}
					KeyCode::Char(c) => {
						app.input.push(c);
					}
					KeyCode::Backspace => {
						app.input.pop();
					}
					KeyCode::Esc => {
						app.input_mode = InputMode::Normal;
					}
					_ => {}
				},
			}
		}*/

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

fn main() {
	let mut terminal = ui::ui_setup().unwrap();

	//ui::update_dashboard(ui);
	run_app(&mut terminal);

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
