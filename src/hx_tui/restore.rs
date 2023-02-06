use std::io::{Stdout, Error};

use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{}
    terminal::disable_raw_mode,
    execute
};

///
/// Restores the user's cli back to the usual prompt
/// 
fn restore_cli(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}