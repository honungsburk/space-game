pub mod events;
mod game;
pub mod misc;
// mod main_menu;
mod systems;

use game::GamePlugin;
// use main_menu::MainMenuPlugin;

use bevy::prelude::*;
use systems::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // .add_state::<AppState>()
        // My Plugins
        // .add_plugin(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup Systems
        .add_systems(Startup, spawn_camera)
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
