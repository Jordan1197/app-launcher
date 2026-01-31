use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub watch_paths: Vec<PathBuf>,
    pub skip_hidden_files: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(toml::from_str(&fs::read_to_string("config.toml")?)?)
    }
}
