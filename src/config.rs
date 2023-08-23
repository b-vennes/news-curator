use crate::types::config::Config;
use std::{fs, path::Path};

impl Config {
    pub fn read_yaml(path: &Path) -> Result<Config, String> {
        let config_data = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{:?}': {:?}", path, e))?;
        return serde_yaml::from_str(&config_data).map_err(|e| e.to_string());
    }
}
