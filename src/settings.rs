use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml::from_str;

/// Settings are settings that the player is allow to change.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub window: WindowSettings,
}

impl Settings {
    pub fn load_from_file(path: &str) -> Result<Settings, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let settings = from_str(&contents)?;
        Ok(settings)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string(&self)?;
        std::fs::write(path, contents)?;
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
