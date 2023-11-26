use crate::{
    file_save::{self, FileSave},
    game::debug::VisualDebug,
    scene::GameScene,
};
use bevy::{prelude::*, window::PrimaryWindow};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, error::Error};
use toml::from_str;

/// Settings are settings that the player is allow to change.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Settings {
    pub scene: Option<GameScene>,
    pub visual_debug: HashSet<VisualDebug>,
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]

pub struct WindowSettings {
    pub resolution: ResolutionSetting,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: ResolutionSetting::default(),
        }
    }
}

/// A plugin that loads the settings from a file and inserts them into the app
/// as resources at start.
pub struct SettingsPlugin(Settings);

impl SettingsPlugin {
    pub fn new(settings: Settings) -> Self {
        Self(settings)
    }
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.window.resolution.clone())
            .add_systems(Update, update_resolution);
    }
}

// Setting Resources

/// What the default resolution should be on start. The player can still resized the window.
/// Resizing the window should not change this setting. They have to manually change it
/// in the settings menu.
///
#[derive(Resource, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResolutionSetting {
    pub x: u32,
    pub y: u32,
}

impl Default for ResolutionSetting {
    fn default() -> Self {
        Self { x: 1280, y: 720 }
    }
}

// TODO: Change to one shot system?
fn update_resolution(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    resolution: Res<ResolutionSetting>,
) {
    if resolution.is_changed() {
        if let Ok(mut window) = window_query.get_single_mut() {
            window
                .resolution
                .set(resolution.x as f32, resolution.y as f32);
        }
    }
}
