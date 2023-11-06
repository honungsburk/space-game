//! # State
//!
//! The state of the game is managed by the `StatePlugin` and the `AppState` enum.
//! Each state is a different "mode" of the game. For example, the `MainMenu` state
//! is the state that is active when the game is first started. The `InGame` state
//! is the state that is active when the player is playing the game. The `Paused`
//! state is the state that is active when the player pauses the game.
mod enemy_ship_ai;
mod main_game;
mod player_death;
mod turret_performance;

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use std::fmt;

use crate::game::input::InputAction;
pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameMode>()
            .init_resource::<Reload>()
            .add_plugins((
                main_game::MainGamePlugin,
                turret_performance::TurretPerformancePlugin,
                player_death::PlayerDeathPlugin,
                enemy_ship_ai::EnemyShipAIPlugin,
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
    MainGame,
    // Debug Game Modes
    TurretPerformance, // Performance testing mode with a lot of turrets
    PlayerDeath,       // Player death testing mode
    #[default]
    EnemyShipAI,
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameMode::None => write!(f, "None"),
            GameMode::MainGame => write!(f, "Main Game"),
            GameMode::TurretPerformance => write!(f, "Turret Performance"),
            GameMode::PlayerDeath => write!(f, "Player Death"),
            GameMode::EnemyShipAI => write!(f, "Enemy Ship AI"),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Resource)]
pub struct Reload(Option<GameMode>);

fn update_game_mode(
    mut reload_game_mode: ResMut<Reload>,
    current_game_mode: Res<State<GameMode>>,
    mut next_game_mode: ResMut<NextState<GameMode>>,
    input_query: Query<&ActionState<InputAction>>,
) {
    if let Ok(input_action) = input_query.get_single() {
        if let Some(mode) = reload_game_mode.0.take() {
            next_game_mode.set(mode);
            reload_game_mode.set_if_neq(Reload(None));
        } else if input_action.just_pressed(InputAction::GameModeNone) {
            next_game_mode.set(GameMode::None);
        } else if input_action.just_pressed(InputAction::GameModeMainGame) {
            next_game_mode.set(GameMode::MainGame); // TODO: we are accidentally closing the window!
        } else if input_action.just_pressed(InputAction::GameModeTurretPerformance) {
            next_game_mode.set(GameMode::TurretPerformance);
        } else if input_action.just_pressed(InputAction::GameModePlayerDeath) {
            next_game_mode.set(GameMode::PlayerDeath);
        } else if input_action.just_pressed(InputAction::GameModeEnemyShipAI) {
            next_game_mode.set(GameMode::EnemyShipAI);
        } else if input_action.just_pressed(InputAction::GameModeReload) {
            reload_game_mode.set_if_neq(Reload(Some(current_game_mode.get().clone())));
            next_game_mode.set(GameMode::None);
        }
    }
}
