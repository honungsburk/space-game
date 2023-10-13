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
use config::Config;
use game::GamePlugin;
use parent_child_no_rotation::NoRotationPlugin;
use settings::Settings;
use systems::*;

// pub fn run(config: Config, settings: Settings) {
pub fn run(_config: Config, _settings: Settings) {
    App::new()
        .insert_resource(Msaa::Sample4)
        // External Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        // Internal Plugins
        .add_plugins(GamePlugin)
        .add_plugins(NoRotationPlugin)
        // Systems
        .add_systems(Update, exit_game)
        .run();
}
