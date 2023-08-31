use crossterm::{
	event::EnableMouseCapture,
	execute,
	terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::io;
use tui::{
	backend::CrosstermBackend,
	layout::{Constraint, Direction, Layout},
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, List, ListItem, ListState, Widget},
	Terminal,
};

pub fn gameselect(
	mut terminal: &Terminal<CrosstermBackend<io::Stdout>>,
	app: App,
) -> Result<(), io::Error> {
	let menu_options = [
		ListItem::new("test"),
		ListItem::new("bigtest"),
		ListItem::new("quit"),
	];

	let list = List::new(menu_options)
		.block(Block::default().title("List").borders(Borders::ALL))
		.style(Style::default().fg(Color::White))
		.highlight_style(Style::default().add_modifier(Modifier::ITALIC))
		.highlight_symbol(">>");

	terminal.draw(|f| {
		let size = f.size();
		f.render_stateful_widget(list, size);
	})?;

    terminal.draw(|f| {
        let size = f.size();
        f.render_stateful_widget(list, size, &mut app.games.state);
    })?;

    Ok(())
}

pub fn update_dashboard(mut terminal: Terminal<CrosstermBackend<io::Stdout>>, update: &str) {
	terminal
		.draw(|f| {
			let size = f.size();
			let block = Block::default()
				.title("My cool block")
				.borders(Borders::ALL);
			f.render_widget(block, size);
		})
		.unwrap();
}

pub fn ui_setup() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
	enable_raw_mode()?;
	let mut stdout = io::stdout();

	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let terminal = Terminal::new(backend)?;

	Ok(terminal)
}
