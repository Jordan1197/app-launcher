use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub watch_paths: Vec<PathBuf>,
    pub watch_extensions: HashSet<String>,
    pub skip_hidden_files: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self, String> {
        let filename = "config.toml";
        let content = fs::read_to_string(filename).map_err(|_| {
            format!(
                "Critical Error: '{}' not found. Please create it in the same folder as the .exe.",
                filename
            )
        })?;

        let config = toml::from_str(&content)
            .map_err(|e| format!("Critical Error: '{}' is invalid. Details: {}", filename, e))?;

        Ok(config)
    }
}
