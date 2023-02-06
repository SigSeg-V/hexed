///
/// Sets up crossterm to work in full-screen mode
/// 
fn setup_cli(backend: CrosstermBackend<Stdout>) -> Result<(Terminal<CrosstermBackend<Stdout>>), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    
    let terminal = Terminal::new(backend)?;
    return Ok(terminal);
}

fn setup_tui() -> Result<(), io::Error> {

    let backend = setup_backend()?;
    let mut terminal = setup_cli(backend)?;

    draw_screen(&mut terminal)?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    restore_cli(&mut terminal)?;
Ok(())
}

fn setup_backend() -> Result<CrosstermBackend<Stdout>, io::Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(CrosstermBackend::new(stdout))
}