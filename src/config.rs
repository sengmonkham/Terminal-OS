#[derive(Debug, Describe, Default)]
pub struct Config {
    pub Workspace: Vec<WorkspaceConfig>,
}

#[derive(Debug, Deserealize, Default)]
pub struct Config {
    pub workspace: Vec<WorkspaceConfig>,
}

#[derive(Debug, Deserealize, Default)]
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
