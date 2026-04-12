use crate::ui::App;
use crate::config::Config;

pub struct AppState {
    pub config: Config,
    pub active_workspace_index: usize,
    pub active_app_index: usize,
    pub is_running: bool,
    pub running_apps: Vec<Box<dyn App>>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            active_workspace_index: 0,
            active_app_index: 0,
            is_running: true,
            running_apps: Vec::new(),
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}
