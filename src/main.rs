mod apps;
mod client;
mod server;
mod ui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // For now, default to launching the client UI.
    // In the future, this will parse CLI args to start the daemon vs client.
    client::run()
}
