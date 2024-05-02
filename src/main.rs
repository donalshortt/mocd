mod db;
mod parser;
mod sender;
mod ui;

extern crate chrono;

use crate::{
    db::{create_gamelisting, GameListing},
    ui::{Setting, StatefulList},
};

use chrono::{
    offset::Utc,
    DateTime
};

use crossterm::{
    event::{
        self, 
        DisableMouseCapture, 
        Event, 
        KeyCode, 
        KeyEvent, 
        KeyEventKind
    },
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

use serde::{
    ser::SerializeStruct,
    Serialize,
    Serializer,
};
use ui::Update;

use std::{
    error::Error, fs::*, io::{self, Read}, net::Ipv4Addr, panic::{catch_unwind, AssertUnwindSafe}, path::{Path, PathBuf}, thread, time::{Duration, Instant}
};

use tui::{
    widgets::ListItem,
    backend::CrosstermBackend,
    Terminal,
};

enum AppState {
	Dashboard,
	GameSelect,
	NewGame,
    Settings,
    Quitting
}

pub struct App<'a> {
	app_state: AppState,
	games: StatefulList<'a>,
	current_game: Option<Game>,
    dashboard_updates: Vec<Update>,
    user_input: String,
    savefile_filepath: PathBuf,
    webserver_ip: Ipv4Addr,
}

impl Default for App<'_> {
	fn default() -> Self {
		Self {
			app_state: AppState::GameSelect,
			games: StatefulList::default(),
			current_game: None,
            dashboard_updates: Vec::new(),
            user_input: String::new(),
            savefile_filepath: get_savefile_path(),
            webserver_ip: Ipv4Addr::new(127, 0, 0, 1),
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
	pub ign: String,
	pub tag: String,
	pub score: u32,
}

impl Default for Player {
	fn default() -> Player {
		Player {
			ign: String::new(),
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
		state.serialize_field("ign", &self.ign)?;
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
        let path = format!("C:\\Users\\{}\\Documents\\Paradox Interactive\\Europa Universalis IV\\save games\\autosave", whoami::username());
		return PathBuf::from(path);
	}

	#[cfg(target_os = "linux")]
	{
        let path = format!("/home/{}/.local/share/Paradox Interactive/Europa Universalis IV/save games/mp_autosave.eu4", whoami::username());
        return PathBuf::from(path);
	}
}

fn run_gameselect(mut app: &mut App, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    terminal
        .draw(|frame| {
            ui::gameselect(frame, &mut app).unwrap();
        })
        .expect("failed to draw gameselect ui");

    if let Event::Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    }) = event::read()?
    {
        match code {
            KeyCode::Char('q') => {
                app.app_state = AppState::Quitting;
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
            KeyCode::Char('s') => {
                app.app_state = AppState::Settings;
            }
            _ => {}
        }
    }

    Ok(())
}

fn run_dashboard(
    app: &mut App, 
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, 
    last_time: &mut String
    ) -> Result<(), io::Error> {
	
    let tick_rate = Duration::from_millis(250);
	let last_tick = Instant::now();

    terminal
        .draw(|frame| {
            ui::dashboard(frame, app.dashboard_updates.clone(), &app).unwrap();
        })
        .expect("failed to draw dashboard ui");

    let mut parsed_data = ParsedGame::default();

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
                    app.app_state = AppState::Quitting;
                }
                KeyCode::Char('s') => {
                    app.app_state = AppState::Settings;
                }
                KeyCode::Esc => {
                    app.app_state = AppState::GameSelect;
                }
                _ => {}
            }
        }
    }

    let latest_metadata = metadata(&app.savefile_filepath)
        .expect("Couldn't get metadata from savefile")
        .modified()
        .expect("Couldn't get time modified from metadata");
    let latest_datetime: DateTime<Utc> = latest_metadata.into();
    let latest_time = latest_datetime.format("%T").to_string();

    if latest_time != *last_time {
        // write to file instead of this variable
        *last_time = latest_time.clone();
        write("last_metadata.txt", latest_time.clone())
            .expect("failed to write time last modified to file");

        parser::parse(&app.savefile_filepath, &mut parsed_data);
        app.current_game.as_mut().unwrap().parsed_game = parsed_data.clone();

        sender::send(&app);

        let update = Update {
            info: String::from("Sent update as of year ".to_string()
            + &parsed_data.date + " at " + &latest_time),
            class: ui::UpdateClass::Info,
        };

        app.dashboard_updates.push(update);

        app.current_game
            .as_mut()
            .unwrap()
            .years_elapsed_this_session += 1;
    } else {
        // TODO: if we press a key and happen to be sleeping in here, the program will feel slow to respond
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn run_newgame(app: &mut App, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    terminal
        .draw(|f| {
            ui::newgame(f, &app.user_input).unwrap();
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
                create_gamelisting(app.user_input.drain(..).collect());
                app.games.items = db::read_games()
                    .iter()
                    .map(|s| ListItem::new(s.to_string()))
                    .collect();
                app.app_state = AppState::GameSelect
            }
            KeyCode::Char(c) => {
                app.user_input.push(c);
            }
            KeyCode::Backspace => {
                app.user_input.pop();
            }
            KeyCode::Esc => app.app_state = AppState::GameSelect,
            _ => {}
        }
    }

    Ok(())
}

fn update_setting(app: &mut App, setting: &Setting) {
    match setting {
        Setting::IP => {
            // TODO: fail more gracefully
            app.webserver_ip = app.user_input.parse().expect("invalid ipv4 address format");
        }
        Setting::Filepath => {
            app.savefile_filepath = PathBuf::from(&app.user_input);

            if let Err(e) = app.savefile_filepath {
                let update = Update {
                    info: String::from("Cannot find savefile"),
                    class: ui::UpdateClass::Warning,
                };

                app.dashboard_updates.push(update);
            }
        }
    }
}

fn run_settings(app: &mut App, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut current_setting: &mut Setting) -> Result<(), io::Error> {
    terminal
        .draw(|frame| {
            ui::settings(frame, &app, &mut current_setting).unwrap();
        })
        .expect("failed to draw settings ui");
    if let Event::Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    }) = event::read()?
    {
        match code {
            KeyCode::Enter => {
                update_setting(app, &current_setting);
                app.user_input.clear();
            }
            KeyCode::Tab => {
                *current_setting = match current_setting {
                    Setting::IP => Setting::Filepath,
                    Setting::Filepath => Setting::IP
                };
                app.user_input.clear();
            }
            KeyCode::Char(c) => {
                app.user_input.push(c);
            }
            KeyCode::Backspace => {
                app.user_input.pop();
            }
            KeyCode::Esc => app.app_state = AppState::GameSelect,
            _ => {}
        }
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
	// TODO: something about this
	db::check_exists();
	let mut app = App::default();
	let mut last_time: String = String::new();
    let mut current_setting = Setting::IP;

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
			AppState::GameSelect => { run_gameselect(&mut app, terminal)?; }
			AppState::Dashboard => { run_dashboard(&mut app, terminal, &mut last_time)?; }
			AppState::NewGame => { run_newgame(&mut app, terminal)?; }
            AppState::Settings => { run_settings(&mut app, terminal, &mut current_setting)?; }
            AppState::Quitting => { return Ok(()); }
		}
	}
}

fn cleanup(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut terminal = ui::ui_setup().expect("Failed to setup UI");
    
    let _ = catch_unwind(AssertUnwindSafe(|| {
        run_app(&mut terminal)
    }));

    cleanup(&mut terminal)?;
    
    Ok(())
}
