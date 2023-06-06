use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};

pub fn update_dashboard(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("My cool block")
            .
            .borders(Borders::ALL);
        f.render_widget(block, size);
    }).unwrap();
}

pub fn ui_setup() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()
        .expect("enable_raw_mode() failed");
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
