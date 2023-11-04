//! # State
//!
//! The state of the game is managed by the `StatePlugin` and the `AppState` enum.
//! Each state is a different "mode" of the game. For example, the `MainMenu` state
//! is the state that is active when the game is first started. The `InGame` state
//! is the state that is active when the player is playing the game. The `Paused`
//! state is the state that is active when the player pauses the game.
mod main_game;
mod player_death;
mod turret_performance;

use bevy::prelude::*;

pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMode>()
            .add_plugins((
                main_game::MainGamePlugin,
                turret_performance::TurretPerformancePlugin,
            ))
            .add_systems(Update, update_game_mode);
    }
}

/// # Mode
///
/// The mode of the game is managed by the `ModePlugin` and the `Mode` enum.
/// Each mode is a different "mode" of the game. For example, the `MainGame` mode
/// is the mode that is activefor normal gameplay. The `TurretPerformance`
/// is to perform performance testing on the turret enemy type.
///
///
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameMode {
    // Real Game Modes
    None, // No mode
    #[default]
    MainGame,
    // Debug Game Modes
    TurretPerformance, // Performance testing mode with a lot of turrets
}

fn update_game_mode(
    mut next_game_mode: ResMut<NextState<GameMode>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F8) {
        next_game_mode.set(GameMode::None);
    } else if keyboard_input.just_pressed(KeyCode::F9) {
        next_game_mode.set(GameMode::MainGame); // TODO: we are accidentally closing the window!
    } else if keyboard_input.just_pressed(KeyCode::F10) {
        next_game_mode.set(GameMode::TurretPerformance);
    }
}
