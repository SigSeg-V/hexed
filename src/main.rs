use std::{io::{self, Stdout}, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal, Frame
};
use tui::terminal::{CompletedFrame};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod hx_tui;




fn main() -> Result<(), io::Error> {
    setup_tui()?;
    Ok(())
}