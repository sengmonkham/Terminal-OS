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

struct TerminalGuard;

impl TerminalGuard {
    fn init() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(Self)
    }
}

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

use crate::state::AppState;

pub fn run(mut app_state: AppState) -> Result<(), Box<dyn Error>> {
    let _guard = TerminalGuard::init()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    while app_state.is_running {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            if app_state.running_apps.is_empty() {
                let empty_block = Block::default()
                    .title("Terminal OS - Workspace")
                    .borders(Borders::ALL);
                f.render_widget(empty_block, chunks[0]);
            } else {
                if let Some(active_app) = app_state.running_apps.get_mut(app_state.active_app_index) {
                    active_app.render(f, chunks[0]);
                }
            }

            let main_block = Block::default()
                .title(" Terminal OS - Workspace ")
                .borders(Borders::ALL);
            f.render_widget(main_block, chunks[0]);

            let status_bar =
                Paragraph::new(" Status: Running | Preloaded apps active | Press 'q' to quit")
                    .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_bar, chunks[1]);
        })?;

        let app_name = if !app_state.running_apps.is_empty() {
            app_state.running_apps[app_state.active_app_index].name()
        } else {
            "None"
        };

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    Ok(())
}
