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
    let config = Config::default();
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
