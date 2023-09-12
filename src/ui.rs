use crate::{db, App};

use crossterm::{
	event::EnableMouseCapture,
	execute,
	terminal::{enable_raw_mode, EnterAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::io;
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Constraint, Direction, Layout},
	style::{Color, Modifier, Style},
	text::{Span, Spans, Text},
	widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
	Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
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
	pub fn new(name: String, time_created: String, last_updated: String, uuid: String) -> Self {
		Self {
			name,
			time_created,
			last_updated,
			uuid,
		}
	}
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

pub fn newgame<B: Backend>(
	f: &mut Frame<B>,
	app: &mut App,
	user_input: &String,
) -> Result<(), io::Error> {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(2)
		.constraints(
			[
				Constraint::Length(1),
				Constraint::Length(3),
				Constraint::Max(1),
			]
			.as_ref(),
		)
		.split(f.size());

	// prepare and render the help message
	let msg = vec![
		Span::raw("Create a game, press "),
		Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
		Span::raw(" to go back."),
	];

	let mut text = Text::from(Spans::from(msg));
	text.patch_style(Style::default());
	let info_message = Paragraph::new(text);
	f.render_widget(info_message, chunks[0]);

	// prepare and render the input box
	let input = Paragraph::new(user_input.as_ref())
		.style(Style::default())
		.block(Block::default().borders(Borders::ALL).title("Input"));
	f.render_widget(input, chunks[1]);

	let blank = Block::default().style(Style::default());
	f.render_widget(blank, chunks[2]);

	f.set_cursor(chunks[1].x + user_input.width() as u16 + 1, chunks[1].y + 1);

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
