#![allow(dead_code)]

pub mod events;
mod game;
pub mod misc;
// mod main_menu;
mod parent_child_no_rotation;
mod systems;

use game::GamePlugin;
// use main_menu::MainMenuPlugin;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use parent_child_no_rotation::NoRotationPlugin;
use systems::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        // .add_state::<AppState>()
        // My Plugins
        // .add_plugin(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(NoRotationPlugin)
        // Startup Systems
        // Systems
        // .add_system(transition_to_game_state)
        // .add_system(transition_to_main_menu_state)
        .add_systems(Update, exit_game)
        // .add_system(handle_game_over)
        .run();
}

// #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
// pub enum AppState {
//     #[default]
//     MainMenu,
//     Game,
//     GameOver,
// }
