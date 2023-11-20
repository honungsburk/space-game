#![allow(dead_code)]

pub mod cli;
pub mod file_save;
pub mod game;
pub mod misc;
mod parent_child_no_rotation;
pub mod prelude;
pub mod scene;
pub mod settings;
mod ui;
pub mod utility_systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use game::{score::high_score, GamePlugin};
use parent_child_no_rotation::NoRotationPlugin;
use scene::ScenePlugin;
use settings::{Settings, SettingsPlugin};
use ui::hud::HudPlugin;
use utility_systems::*;

// pub fn run(config: Config, settings: Settings) {
pub fn run(settings: Settings, _high_scores: high_score::HighScores) {
    let mut app = App::new();

    // Defaults
    app.add_plugins(DefaultPlugins);

    // Add 2D drawing Plugin
    app.insert_resource(Msaa::Sample4).add_plugins(ShapePlugin);

    // Add Internal Plugins
    app.add_plugins(GamePlugin)
        .add_plugins(NoRotationPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(ScenePlugin)
        .add_plugins(SettingsPlugin::new(settings))
        // Systems
        .add_systems(Update, exit_game);

    app.run()
}
