use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend}, 
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    Frame,
};
use crossterm::{
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// game
use lib_game::Game;

fn main() -> Result<(), io::Error> {
    let game: Game = lib_game::Game::new();
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui);

    // read user input
    loop {
        match read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => { break; },
            _ => (),
        }
    }
    
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

use self::game_widget::*;

mod game_widget;

// main ui
fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(f.size());
    let block = basic_block("Stats");
    f.render_widget(block, chunks[0]);
    let block = basic_block("Game");    
    f.render_widget(block, chunks[1]);
    let block = basic_block("Pieces");
    f.render_widget(block, chunks[2]);

    let game_widget = game_widget::get_widget();

    let game_layout = Layout::default()
        .margin(20)
        .constraints(
            [
                Constraint::Percentage(100),
            ].as_ref()
        )
        .split(chunks[1]);

    f.render_widget(game_widget, game_layout[0]);
}

pub fn basic_block(title: &'static str) -> Block<'static> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
}
