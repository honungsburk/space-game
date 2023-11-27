//!
//! TODO: Settings and flags (in teh config module) are representing the same thing.
//! Need to merge them.
//!

use crate::{
    file_save::{self, FileSave},
    game::debug::VisualDebug,
    scene::GameScene,
};
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};
use clap::ValueEnum;
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
    pub selection: Option<u32>,
    pub mode: Option<WindowModeSetting>,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: ResolutionSetting::default(),
            selection: None,
            mode: None,
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
            .insert_resource(MonitorSelectionSetting {
                selection: self
                    .0
                    .window
                    .selection
                    .map(|n| MonitorSelection::Index(n as usize))
                    .unwrap_or(MonitorSelection::Primary),
            })
            .insert_resource(self.0.window.mode.unwrap_or_default())
            .add_systems(
                Update,
                (
                    update_resolution,
                    update_monitor_selection,
                    update_window_mode,
                ),
            );
    }
}

// Setting Resources
#[derive(Resource, Debug, PartialEq, Eq, Clone)]
pub struct MonitorSelectionSetting {
    selection: MonitorSelection,
}

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

/// TODO: There is a bug in bevy that prevents this from working.
/// See https://github.com/bevyengine/bevy/issues/7600
fn update_monitor_selection(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    monitor_selection: Res<MonitorSelectionSetting>,
) {
    if monitor_selection.is_changed() {
        if let Ok(mut window) = window_query.get_single_mut() {
            println!("Monitor Selection: {:?}", monitor_selection.selection);
            window.position = WindowPosition::Centered(monitor_selection.selection);
        }
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

///
/// What mode to use for the game window.
///
/// Had to implement this because the `WindowMode` enum doesn't implement `ValueEnum`, `Serialize`, and `Deserialize`.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, ValueEnum, Serialize, Deserialize,
)]
pub enum WindowModeSetting {
    #[default]
    Windowed,
    BorderlessFullscreen,
    SizedFullscreen,
    Fullscreen,
}

impl WindowModeSetting {
    pub fn to_window_mode(&self) -> WindowMode {
        match self {
            WindowModeSetting::Windowed => WindowMode::Windowed,
            WindowModeSetting::BorderlessFullscreen => WindowMode::BorderlessFullscreen,
            WindowModeSetting::SizedFullscreen => WindowMode::SizedFullscreen,
            WindowModeSetting::Fullscreen => WindowMode::Fullscreen,
        }
    }
}

fn update_window_mode(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    resolution: Res<WindowModeSetting>,
) {
    if resolution.is_changed() {
        if let Ok(mut window) = window_query.get_single_mut() {
            window.mode = resolution.to_window_mode();
        }
    }
}
