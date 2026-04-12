use clap::{Parser, Subcommand};
mod apps;
mod client;
mod config;
mod server;
mod state;
mod ui;

use config::Config;
use state::AppState;
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    //strat the background commands
    Daemon,
    Client,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    //load from file
    let config_str = fs::read_to_string("config.toml").unwrap_or_else(|_| "".to_string());
    let config = if config_str.is_empty() {
        Config::default()
    } else {
        Config::load(&config_str)
    };
    let app_state = AppState::new(config);

    match &cli.command {
        Some(Commands::Daemon) => {
            //bg server
            server::start_daemon();
            Ok(())
        }
        Some(Commands::Client) | None => client::run(app_state),
    }
}
