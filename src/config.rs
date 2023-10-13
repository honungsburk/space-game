//! Configuration for the game.
//!
//! Contains all options that are usefull during development.
//!
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml::from_str;

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

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config = from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string(&self)?;
        std::fs::write(path, contents)?;
        Ok(())
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
