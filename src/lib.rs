#![allow(dead_code)]

pub mod cli;
pub mod config;
pub mod file_save;
pub mod game;
pub mod misc;
mod parent_child_no_rotation;
pub mod settings;
mod systems;
mod ui;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use config::{Config, VisualDebug};
use game::{score::high_score, GamePlugin};
use parent_child_no_rotation::NoRotationPlugin;
use settings::Settings;
use systems::*;
use ui::hud::HudPlugin;

// pub fn run(config: Config, settings: Settings) {
pub fn run(config: Config, _settings: Settings, high_scores: high_score::HighScores) {
    let mut app = App::new();

    // Defaults
    app.add_plugins(DefaultPlugins);

    // Add 2D drawing Plugin
    app.insert_resource(Msaa::Sample4).add_plugins(ShapePlugin);

    // Add Internal Plugins
    app.add_plugins(GamePlugin {
        has_camera_debug: config.has_visual_debug(VisualDebug::Camera),
        has_colliders_debug: config.has_visual_debug(VisualDebug::Colliders),
        high_scores,
    })
    .add_plugins(NoRotationPlugin)
    .add_plugins(HudPlugin)
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    // Systems
    .add_systems(Update, exit_game);

    app.run()
}
