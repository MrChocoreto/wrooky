mod draw_ui;
mod keyevents;
// use ratatui::{backend::Backend};
// use crossterm::event::{read, Event, KeyCode};
// use tui_textarea::{TextArea, Input, Key};


use std::io;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(draw_ui::ui)?;
        should_quit = keyevents::handle_events()?;
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
