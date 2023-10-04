use crate::{db, App};

use crossterm::{
	event::EnableMouseCapture,
	execute,
	terminal::{enable_raw_mode, EnterAlternateScreen},
};
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

pub fn newgame<B: Backend> (
	frame: &mut Frame<B>,
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
		.split(frame.size());

	// prepare and render the help message
	let msg = vec![
		Span::raw("Create a game, press "),
		Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
		Span::raw(" to go back."),
	];

	let mut text = Text::from(Spans::from(msg));
	text.patch_style(Style::default());
	let info_message = Paragraph::new(text);
	frame.render_widget(info_message, chunks[0]);

	// prepare and render the input box
	let input = Paragraph::new(user_input.as_ref())
		.style(Style::default())
		.block(Block::default().borders(Borders::ALL).title("Input"));
	frame.render_widget(input, chunks[1]);

	let blank = Block::default().style(Style::default());
	frame.render_widget(blank, chunks[2]);

	frame.set_cursor(chunks[1].x + user_input.width() as u16 + 1, chunks[1].y + 1);

	Ok(())
}

pub fn dashboard<B: Backend> (
	frame: &mut Frame<B>,
	mut updates: Vec<String>,
    app: &App
) -> Result<(), io::Error> {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Min(1),
            ]
            .as_ref()
        )
        .split(frame.size());
    // TODO: have a info box displayed at the time: the game name basically
    
    let banner = Block::default()
        .title("Overview")
        .borders(Borders::ALL);

    let name = Spans::from(vec![
        Span::styled("Game name: ", Style::default().add_modifier(Modifier::BOLD)),
        // TODO: instead of unrapping like a pleb, pattern match here!
        // then also make sure to move the code that sets the current id in main, back to it's
        // original home (after the terminal draw)
        // pattern matching here might not be necessary
        Span::from(
            app.current_game.as_ref().unwrap().name.clone()
        )]
    );

    

    let text = Paragraph::new(vec![name]).block(banner);
    frame.render_widget(text, chunks[0]);

    
    // in the unlikely event that this program runs for 1000 years and our list gets very big, this
    // rotating log has got us covered
    let list_item_height = 4;
    let max_displayable_items = chunks[1].height / list_item_height;

    if &updates.len() > &(max_displayable_items as usize) {
        updates.remove(0);
    }

    let pretty_updates: Vec<ListItem> = 
        updates
        .iter()
        .rev()
        .map(|update|{
            
            let header = Spans::from(
                Span::styled(format!("{:<9}", "INFO"), Style::default().fg(Color::Blue))
            );
            //let body = Spans::from("wowee");
            let body = Spans::from(update.clone());

            ListItem::new(vec![
                Spans::from("-".repeat(chunks[1].width as usize)),
                header,
                Spans::from(""),
                body
            ])
        })
        .collect();

	let list = List::new(pretty_updates)
		.block(Block::default().title("Updates").borders(Borders::ALL))
		.style(Style::default().fg(Color::White));

	frame.render_widget(list, chunks[1]);

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
