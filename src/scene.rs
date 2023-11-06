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
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Scene>()
            .init_resource::<Reload>()
            .add_plugins((
                main_game::MainGameScenePlugin,
                turret_performance::TurretPerformanceScenePlugin,
                player_death::PlayerDeathScenePlugin,
                enemy_ship_ai::EnemyShipAIScenePlugin,
            ))
            .add_systems(Update, update_scene);
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
pub enum Scene {
    // Real Game Modes
    None, // No mode
    MainGame,
    // Debug Game Modes
    TurretPerformance, // Performance testing mode with a lot of turrets
    PlayerDeath,       // Player death testing mode
    #[default]
    EnemyShipAI,
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scene::None => write!(f, "None"),
            Scene::MainGame => write!(f, "Main Game"),
            Scene::TurretPerformance => write!(f, "Turret Performance"),
            Scene::PlayerDeath => write!(f, "Player Death"),
            Scene::EnemyShipAI => write!(f, "Enemy Ship AI"),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Resource)]
pub struct Reload(Option<Scene>);

fn update_scene(
    mut reload_scene: ResMut<Reload>,
    current_scene: Res<State<Scene>>,
    mut next_scene: ResMut<NextState<Scene>>,
    input_query: Query<&ActionState<InputAction>>,
) {
    if let Ok(input_action) = input_query.get_single() {
        if let Some(mode) = reload_scene.0.take() {
            next_scene.set(mode);
            reload_scene.set_if_neq(Reload(None));
        } else if input_action.just_pressed(InputAction::SceneNone) {
            next_scene.set(Scene::None);
        } else if input_action.just_pressed(InputAction::SceneMainGame) {
            next_scene.set(Scene::MainGame);
        } else if input_action.just_pressed(InputAction::SceneTurretPerformance) {
            next_scene.set(Scene::TurretPerformance);
        } else if input_action.just_pressed(InputAction::ScenePlayerDeath) {
            next_scene.set(Scene::PlayerDeath);
        } else if input_action.just_pressed(InputAction::SceneEnemyShipAI) {
            next_scene.set(Scene::EnemyShipAI);
        } else if input_action.just_pressed(InputAction::SceneReload) {
            reload_scene.set_if_neq(Reload(Some(current_scene.get().clone())));
            next_scene.set(Scene::None);
        }
    }
}
