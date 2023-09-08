use crate::{db, App};

use crossterm::{
	event::EnableMouseCapture,
	execute,
	terminal::{enable_raw_mode, EnterAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::io;
use tui::{
	backend::CrosstermBackend,
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, List, ListItem, ListState},
	Terminal, layout::{Constraint, Direction, Layout},
};
use uuid::Uuid;

pub struct StatefulList<'a> {
	pub state: ListState,
	pub items: Vec<ListItem<'a>>,
}

impl Default for StatefulList<'_> {
	fn default() -> Self {
		StatefulList {
			state: ListState::default(),
			//TODO: create a function to get the list items from a datafile
			items: db::read_games()
				.iter()
				.map(|s| ListItem::new(s.to_string()))
				.collect(),
		}
	}
}

impl StatefulList<'_> {
	pub fn next(&mut self) {
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

	pub fn previous(&mut self) {
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

// TODO: check if I can make the updated, created and uuid strs
#[derive(Serialize, Deserialize, Debug)]
pub struct GameListing {
	pub(crate) name: String,
	pub(crate) time_created: String,
	pub(crate) last_updated: String,
	pub(crate) uuid: String,
}

impl GameListing {
    pub fn new(name: String, time_created: String, last_updated: String, uuid: String) -> Self { Self { name, time_created, last_updated, uuid } }
}

pub fn gameselect(
	terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
	app: &mut App,
) -> Result<(), io::Error> {
	let list = List::new(&*app.games.items)
		.block(Block::default().title("List").borders(Borders::ALL))
		.style(Style::default().fg(Color::White))
		.highlight_style(Style::default().add_modifier(Modifier::ITALIC))
		.highlight_symbol(">>");

	terminal.draw(|f| {
		let size = f.size();
		f.render_stateful_widget(list, size, &mut app.games.state);
	})?;

	Ok(())
}

// to control the layout im gonna have to wrap all of these things in a terminal draw, assuming i
// want to use closures -> maybe find a nicer way to do this so that i don't have so much
// indentation
//

pub fn newgame(
	terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
	app: &mut App,
) -> Result<(), io::Error> {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

	Ok(())
}

pub fn ui_setup() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
	enable_raw_mode()?;
	let mut stdout = io::stdout();

	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let terminal = Terminal::new(backend)?;

	Ok(terminal)
}
