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
use serde::{Serialize, Serializer};
use std::error::Error;
use std::time::{Duration, Instant};
use std::{fs, thread, time};
use std::fs::*;
use std::io::{self, stderr, Write};
use std::io::Read;
use std::path::Path;
use tui::widgets::ListItem;
use ui::GameListing;

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

#[derive(Debug)]
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

#[derive(Debug)]
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


// press "enter" on an item to enter the dashboard
// display, as list, each time an update is sent
// include time sent && which year was sent.

// maybe include an options page?
//
// maybe include a welcome page?
//
// 

fn create_gamelisting(name: String) {
	let current_date = Local::now().date_naive();

	let listing = GameListing {
		name,
		time_created: current_date.to_string(),
		last_updated: current_date.to_string(),
		uuid: Uuid::new_v4().to_string(),
	};

	//TODO: is it really necessary to read and write here?
	let mut listings = db::read_listings();
	listings.push(listing);
	db::write_listings(listings);
}

fn get_game_id(selected_index: usize) -> Option<String> {
    let listings: Vec<GameListing> = db::read_listings();

    if selected_index < listings.len() {
        Some(listings[selected_index].uuid)
    } else {
        None
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
	//TODO: something about this
    db::check_exists();
	let mut app = App::default();
	let mut user_input = String::new();
    let mut selected_index = Some(0);
    let mut updates : Vec<ListItem> = Vec::new();
    let mut game_data = Game::default();
    let filepath = "/home/donal/projects/moc/mocp/saves/mp_autosave.eu4";
    let mut last_time: String = String::new();
    let tick_rate = Duration::from_millis(250);
    let last_tick = Instant::now();


    let mut file: File;
    if !Path::new("last_metadata.txt").exists() {
        File::create("last_metadata.txt")
            .expect("failed to create last file to record time of last metadata access");
    }

    file = File::open("last_metadata.txt")
        .expect("Unable to open file to read time of last metadata access");
    file.read_to_string(&mut last_time).unwrap();

	loop {
		match app.app_state {
			AppState::GameSelect => {
				ui::gameselect(terminal, &mut app)
					.expect("unable to display game selection screen");

				if let Event::Key(key) = event::read()? {
					match key.code {
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
                            selected_index = app.games.state.selected();
                            game_data.id = get_game_id(selected_index.unwrap()).unwrap();
                            app.app_state = AppState::Dashboard;
                        }
						_ => {}
					}
				}
			}

			AppState::Dashboard => {
                ui::dashboard(terminal, updates.clone())
                    .expect("failed to display dashboard ui");


                // it seems that there is a buffer of inputs that get dealt with in a LIFO manner,
                // as a result i can spam a bunch of input and then finally "q" and it will take
                // forever to quit the program
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if crossterm::event::poll(timeout).unwrap() {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }

                       
                // for now just focus on adding a Sent! message to the list
                // a vec of ListItem

                // gets the latest time modified from the save location
                // gets the latest time right now 
                // converts it all into a var called latest time 

                // the point here is to send only one update per year. if we send multiple we will
                // duplicate years and the frontend does not check for this
                //
                // a file is created to hold the time that the savefile was last modified.
                // we use a file and not just some var in case the program crashes, again because
                // if we sent a duplicate update by accident, remedying this would be annoying

                let latest_metadata = fs::metadata(filepath)
                    .expect("Couldn't get metadata from savefile")
                    .modified()
                    .expect("Couldn't get time modified from metadata");
                let latest_datetime: DateTime<Utc> = latest_metadata.into();
                let latest_time = latest_datetime.format("%T").to_string();

                if latest_time != last_time {
                    // write to file instead of this variable
                    last_time = latest_time.clone();
                    fs::write("last_metadata.txt", latest_time.clone())
                        .expect("failed to write time last modified to file");

                    parser::parse(filepath, &mut game_data);
                    sender::send(&game_data);
                    
                    updates.push(ListItem::new("Sent info for year ".to_string() + &game_data.date + " at " + &latest_time));
                } else {

                    //TODO: if we press a key and happen to be sleeping in here, the program will
                    //feel slow to respond
                    updates.push(ListItem::new("Sleeping!"));
                    thread::sleep(Duration::from_secs(5));
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
				terminal.draw(|f| {ui::newgame(f, &mut app, &user_input).unwrap();})
					.expect("failed to draw newgame ui");

				if let Event::Key(key) = event::read()? {
					match key.code {
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
