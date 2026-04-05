use crate::config::Config;

pub struct AppState {
    pub config: Config,
    pub active_workspace_index: usize,
    pub is_running: bool,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            active_workspace_index: 0,
            is_running: true,
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}
