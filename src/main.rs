mod db;
mod parser;
mod sender;
mod ui;

extern crate chrono;

use crate::db::GameListing;
use crate::ui::StatefulList;
use chrono::offset::Utc;
use chrono::DateTime;
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use db::create_gamelisting;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::error::Error;
use std::fs::*;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{fs, thread};
use tui::widgets::ListItem;

use tui::{backend::CrosstermBackend, Terminal};

enum AppState {
	Dashboard,
	GameSelect,
	NewGame,
}

pub struct App<'a> {
	app_state: AppState,
	games: StatefulList<'a>,
	current_game: Option<Game>,
}

impl Default for App<'_> {
	fn default() -> Self {
		Self {
			app_state: AppState::GameSelect,
			games: StatefulList::default(),
			current_game: None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ParsedGame {
	pub date: String,
	pub players: Vec<Player>,
}

impl Default for ParsedGame {
	fn default() -> Self {
		Self {
			date: String::new(),
			players: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub struct Game {
	parsed_game: ParsedGame,
	years_elapsed_this_session: u16,
	name: String,
	id: String,
}

impl Default for Game {
	fn default() -> Self {
		Game {
			parsed_game: ParsedGame::default(),
			years_elapsed_this_session: 0,
			name: String::new(),
			id: String::new(),
		}
	}
}

#[derive(Debug, Clone)]
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

// TODO: make these both methods of Game? or do they make more sense as functions in DB?
pub fn get_game_id(selected_index: usize) -> Option<String> {
	let listings: Vec<GameListing> = db::read_listings();

	if selected_index < listings.len() {
		Some(listings[selected_index].uuid.clone())
	} else {
		None
	}
}

pub fn get_game_name(selected_index: usize) -> Option<String> {
	let listings: Vec<GameListing> = db::read_listings();

	if selected_index < listings.len() {
		Some(listings[selected_index].name.clone())
	} else {
		None
	}
}

fn get_savefile_path() -> PathBuf {
	#[cfg(target_os = "windows")]
	{
		return PathBuf::from("C:\\Users\\donal\\Documents\\Paradox Interactive\\Europa Universalis IV\\save games\\autosave");
	}

	#[cfg(target_os = "linux")]
	{
		return PathBuf::from("/home/donal/projects/moc/mocp/saves/mp_autosave.eu4");
	}
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
	//TODO: something about this
	db::check_exists();
	let mut app = App::default();
	let mut user_input = String::new();
	let mut dashboard_updates: Vec<String> = Vec::new();
	let mut last_time: String = String::new();
	let tick_rate = Duration::from_millis(250);
	let last_tick = Instant::now();

	let savefile_filepath = get_savefile_path();

	let mut file: File;
	if !Path::new("last_metadata.txt").exists() {
		File::create("last_metadata.txt")
			.expect("failed to create last file to record time of last metadata access");
	}

	file = File::open("last_metadata.txt")
		.expect("failed to open file to read time of last metadata access");
	file.read_to_string(&mut last_time).unwrap();

	loop {
		match app.app_state {
			AppState::GameSelect => {
				ui::gameselect(terminal, &mut app)
					.expect("failed to display game selection screen");

				if let Event::Key(KeyEvent {
					code,
					kind: KeyEventKind::Press,
					..
				}) = event::read()?
				{
					match code {
						KeyCode::Char('q') => {
							return Ok(());
						}
						KeyCode::Char('c') => {
							app.app_state = AppState::NewGame;
						}
						KeyCode::Down => {
							app.games.next();
						}
						KeyCode::Up => {
							app.games.previous();
						}
						KeyCode::Enter => {
							app.current_game = Some(Game::default());
							app.current_game.as_mut().unwrap().name = get_game_name(
								app.games
									.state
									.selected()
									.expect("failed to find selected game"),
							)
							.expect("failed to find game name");
							app.current_game.as_mut().unwrap().id = get_game_id(
								app.games
									.state
									.selected()
									.expect("failed to find selected game"),
							)
							.expect("failed to find game id");
							app.app_state = AppState::Dashboard;
						}
						_ => {}
					}
				}
			}

			AppState::Dashboard => {
				//dbg!(&app.current_game);

				terminal
					.draw(|frame| {
						ui::dashboard(frame, dashboard_updates.clone(), &app).unwrap();
					})
					.expect("failed to draw dashboard ui");

				let mut parsed_data = ParsedGame::default();
				//TODO: pattern match here

				// it seems that there is a buffer of inputs that get dealt with in a LIFO manner,
				// as a result i can spam a bunch of input and then finally "q" and it will take
				// forever to quit the program
				let timeout = tick_rate
					.checked_sub(last_tick.elapsed())
					.unwrap_or_else(|| Duration::from_secs(0));
				if crossterm::event::poll(timeout).unwrap() {
					if let Event::Key(KeyEvent {
						code,
						kind: KeyEventKind::Press,
						..
					}) = event::read()?
					{
						match code {
							KeyCode::Char('q') => {
								return Ok(());
							}
							_ => {}
						}
					}
				}

				// the point here is to send only one update per year. if we send multiple we will
				// duplicate years and the frontend does not check for this
				//
				// a file is created to hold the time that the savefile was last modified.
				// we use a file and not just some var in case the program crashes, again because
				// if we sent a duplicate update by accident, remedying this would be annoying

                //dbg!(&savefile_filepath);
                
				let latest_metadata = fs::metadata(&savefile_filepath)
					.expect("Couldn't get metadata from savefile")
					.modified()
					.expect("Couldn't get time modified from metadata");
				let latest_datetime: DateTime<Utc> = latest_metadata.into();
				let latest_time = latest_datetime.format("%T").to_string();

                //dbg!(&latest_metadata);

				if latest_time != last_time {
					// write to file instead of this variable
					last_time = latest_time.clone();
					fs::write("last_metadata.txt", latest_time.clone())
						.expect("failed to write time last modified to file");

					parser::parse(&savefile_filepath, &mut parsed_data);
					app.current_game.as_mut().unwrap().parsed_game = parsed_data.clone();

					sender::send(
						&app.current_game
							.as_ref()
							.expect("failed to find current game data to send"),
					);

					dashboard_updates.push(String::from(
						"Sent update for year ".to_string()
							+ &parsed_data.date + " at " + &latest_time,
					));

					app.current_game
						.as_mut()
						.unwrap()
						.years_elapsed_this_session += 1;
				} else {
					//TODO: if we press a key and happen to be sleeping in here, the program will
					//feel slow to respond
					thread::sleep(Duration::from_secs(1));
				}
			}

			// i want to assign a uuid to each game that is created, instead of getting it from the
			// parser
			// now that i create a gamelisting in database, i need a way of grabbing the uuid from
			// listings and putting it in the object that the sender sends
			// given that i have the selected index, i should just be able to access the db then
			// select the gamelisting based on this index
			// i want to load this uuid into the Game struct when i select a game
			AppState::NewGame => {
				terminal
					.draw(|f| {
						ui::newgame(f, &user_input).unwrap();
					})
					.expect("failed to draw newgame ui");

				if let Event::Key(KeyEvent {
					code,
					kind: KeyEventKind::Press,
					..
				}) = event::read()?
				{
					match code {
						KeyCode::Enter => {
							create_gamelisting(user_input.drain(..).collect());
							app.games.items = db::read_games()
								.iter()
								.map(|s| ListItem::new(s.to_string()))
								.collect();
							app.app_state = AppState::GameSelect
						}
						KeyCode::Char(c) => {
							user_input.push(c);
						}
						KeyCode::Backspace => {
							user_input.pop();
						}
						KeyCode::Esc => app.app_state = AppState::GameSelect,
						_ => {}
					}
				}
			}
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut terminal = ui::ui_setup().unwrap();

	run_app(&mut terminal).expect("app failed to start");

	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	terminal.show_cursor()?;

	Ok(())
}
