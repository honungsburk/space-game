pub mod events;
mod game;
pub mod misc;

mod parent_child_no_rotation;
mod systems;

use game::GamePlugin;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use parent_child_no_rotation::NoRotationPlugin;
use systems::*;

pub fn run() {
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
