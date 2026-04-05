use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub workspace: Vec<WorkspaceConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct WorkspaceConfig {
    pub name: String,
    pub preload_apps: Vec<String>,
}

impl Config {
    //load the config from a TOML string
    pub fn load(toml_str: &str) -> Self {
        toml::from_str(toml_str).unwrap_or_default()
    }
}
