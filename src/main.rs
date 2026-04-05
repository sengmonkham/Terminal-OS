mod apps;
mod client;
mod config;
mod server;
mod state;
mod ui;

use std::error::Error;
use config::Config;
use state::AppState;

fn main() -> Result<(), Box<dyn Error>> {
    // For now, default to launching the client UI.
    // In the future, this will parse CLI args to start the daemon vs client.
    let config = Config::default();
    let app_state = AppState::new(config);
    client::run(app_state)
}
