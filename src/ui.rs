use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction},
    Terminal, 
    style::{Style, Color, Modifier}
};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};

pub fn gameselect(mut terminal: &Terminal<CrosstermBackend<io::Stdout>>, app: App) -> Result<(), io::Error> {

    let menu_options = [ListItem::new("test"), ListItem::new("bigtest"), ListItem::new("quit")];

    let list = List::new(menu_options)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    terminal.draw(|f| {
        let size = f.size();
        f.render_stateful_widget(list, size);
    })?;

    Ok(())
}

pub fn update_dashboard(mut terminal: Terminal<CrosstermBackend<io::Stdout>>, update: &str) {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("My cool block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    }).unwrap();
}

pub fn ui_setup() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
