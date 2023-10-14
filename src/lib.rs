#![allow(dead_code)]

pub mod cli;
pub mod config;
pub mod events;
mod game;
pub mod misc;
mod parent_child_no_rotation;
pub mod settings;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use config::{Config, VisualDebug};
use game::GamePlugin;
use parent_child_no_rotation::NoRotationPlugin;
use settings::Settings;
use systems::*;

// pub fn run(config: Config, settings: Settings) {
pub fn run(config: Config, _settings: Settings) {
    let mut app = App::new();

    // Defaults
    app.add_plugins(DefaultPlugins);

    // Add Physics Plugin
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    })
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    // Add 2D drawing Plugin
    app.insert_resource(Msaa::Sample4).add_plugins(ShapePlugin);

    // Add Internal Plugins

    app.add_plugins(GamePlugin)
        .add_plugins(NoRotationPlugin)
        // Systems
        .add_systems(Update, exit_game);

    // Add Visual Debugging

    if config.has_visual_debug(VisualDebug::Colliders) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    app.run()
}
