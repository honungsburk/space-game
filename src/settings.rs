use crate::file_save::{self, FileSave};
use serde::{Deserialize, Serialize};
use std::error::Error;
use toml::from_str;

/// Settings are settings that the player is allow to change.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub window: WindowSettings,
}

impl FileSave for Settings {
    type Item = Settings;
    fn load_from_file(path: &str) -> Result<Settings, Box<dyn Error>> {
        let contents = file_save::load_from_file(path)?;
        let settings = from_str(&contents)?;
        Ok(settings)
    }

    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string(&self)?;
        file_save::save_to_file(path, &contents)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]

pub struct WindowSettings {
    pub resolution: (u32, u32),
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: (1280, 720),
        }
    }
}
