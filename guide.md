use crossterm::{
    cursor::Show,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::{error::Error, io};

/// RAII guard to ensure the terminal is always reset exactly once,
/// even if the application panics.
struct TerminalGuard;

impl TerminalGuard {
    fn init() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(Self)
    }
}

/// The Drop trait is the secret sauce for the Terminal Guard.
/// Whenever `TerminalGuard` goes out of scope (even during a panic!),
/// this code will automatically run to restore your terminal.
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        );
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // 1. Set up the terminal using the guard.
    let _guard = TerminalGuard::init()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            let main_block = Block::default()
                .title(" Terminal OS - Workspace ")
                .borders(Borders::ALL);
            f.render_widget(main_block, chunks[0]);

            let status_bar =
                Paragraph::new(" Status: Running | Preloaded apps active | Press 'q' to quit")
                    .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_bar, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    // Notice how we don't have to manually disable raw mode
    // or leave the alternate screen anymore! 
    // `_guard` falls out of scope here and `Drop` cleans it up.

    Ok(())
}
