use anyhow::anyhow;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::file_manager::FileManager;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub openai_api_key: Option<String>,
}

impl Config {
    fn default() -> Self {
        Config {
            openai_api_key: None,
        }
    }

    pub fn load(file_manager: &FileManager) -> Result<Self, anyhow::Error> {
        let config_file_path = String::from("config/main.json");
        match file_manager.file_exists(&config_file_path) {
            Ok(true) => {
                let config_json = file_manager.read_from_file(&config_file_path)?;
                serde_json::from_str(&config_json).or_else(|e| {
                  error!("Config file at '{}' had invalid json:\n{:?}\nCreating new default config and overwriting.", &config_file_path, e);
                  let default_config = Config::default();
                  default_config.save(file_manager)?;
                  Ok(default_config)
                })
            }
            Ok(false) => {
                info!(
                    "No config file found at '{}'. Creating new default config.",
                    &config_file_path
                );
                let default_config = Config::default();
                default_config.save(file_manager)?;
                Ok(default_config)
            }
            _ => Err(anyhow!(
                "Unable to verify existence of config file. Failed to load."
            )),
        }
    }

    pub fn save(&self, file_manager: &FileManager) -> Result<(), anyhow::Error> {
        let config_file_path = String::from("config/main.json");
        let config_json = serde_json::to_string(self)?;
        file_manager.write_to_file(&config_file_path, &config_json)?;
        Ok(())
    }
}
