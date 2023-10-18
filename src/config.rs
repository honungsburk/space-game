//! Configuration for the game.
//!
//! Contains all options that are usefull during development.
//!
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use toml::from_str;

use crate::file_save;
use crate::file_save::FileSave;

/// Configurations are settings that the player is not allow to change.
///
///
/// # Examples
///
/// ```
/// use space_game::config::Config;
///
/// let config = Config::default();
/// ```
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Config {
    pub log_level: LogLevel,
    pub log_file: Option<String>,
    pub visual_debug: HashSet<VisualDebug>,
    pub scene: Scene,
}

impl FileSave for Config {
    type Item = Config;
    fn load_from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let contents = file_save::load_from_file(path)?;
        let config = from_str(&contents)?;
        Ok(config)
    }

    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string(&self)?;
        file_save::save_to_file(path, &contents)?;
        Ok(())
    }
}

impl Config {
    pub fn has_visual_debug(&self, visual_debug: VisualDebug) -> bool {
        self.visual_debug.contains(&visual_debug)
    }
}

/// The LogLevel for the game. Usefull for debugging and development.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, ValueEnum)]
pub enum LogLevel {
    None,
    #[default]
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

///
/// Contains all the options for visual debugging.
///
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum VisualDebug {
    Colliders,
    Camera,
    FPS,
}

/// The current game scene.
///
/// Most of the time this will be `Scene::Game`. However, during development
/// you may want to switch to one of the other scenes for quicker development.
///
/// **Player Accessible**
/// - `Scene::Game` - The main game scene.
/// - `Scene::MainMenu` - The main menu scene.
///
/// **Dev Only**
/// - `Scene::Assets` - All assets in the game laid out in a grid. Useful with
/// `VisualDebug::Colliders` to see the hitboxes. Move around with the arrow keys and zoom with the mouse wheel.
/// - `Scene::EnemyTurret` - Player vs enemy turret
///
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, ValueEnum)]
pub enum Scene {
    // Player Accessible
    #[default]
    MainMenu,
    Game,
    // Dev Only
    Assets,
    EnemeyTurret,
}
