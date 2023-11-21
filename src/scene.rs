//! # State
//!
//! The state of the game is managed by the `StatePlugin` and the `AppState` enum.
//! Each state is a different "mode" of the game. For example, the `MainMenu` state
//! is the state that is active when the game is first started. The `InGame` state
//! is the state that is active when the player is playing the game. The `Paused`
//! state is the state that is active when the player pauses the game.
mod assets;
mod boid;
mod enemy_ship_ai;
mod kamikaze_drone;
mod main_game;
mod player_death;
mod player_movement;
mod turret;
mod turret_performance;

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use std::fmt;

use crate::game::input::InputAction;
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameScene>()
            .init_resource::<Reload>()
            .add_plugins((
                assets::AssetsScenePlugin,
                boid::BoidScenePlugin,
                kamikaze_drone::KamikazeDroneScenePlugin,
                turret::TurretScenePlugin,
                player_movement::PlayerMovementScenePlugin,
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States, Reflect)]
pub enum GameScene {
    // Real Game Modes
    None, // No mode
    MainGame,
    // Debug Game Modes
    TurretPerformance, // Performance testing mode with a lot of turrets
    PlayerDeath,       // Player death testing mode
    EnemyShipAI,
    PlayerMovement,
    Turret,
    KamikazeDrone,
    Boid,
    #[default]
    Assets,
}

impl fmt::Display for GameScene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameScene::None => write!(f, "None"),
            GameScene::MainGame => write!(f, "Main Game"),
            GameScene::TurretPerformance => write!(f, "Turret Performance"),
            GameScene::PlayerDeath => write!(f, "Player Death"),
            GameScene::EnemyShipAI => write!(f, "Enemy Ship AI"),
            GameScene::PlayerMovement => write!(f, "Player Movement"),
            GameScene::Turret => write!(f, "Turret"),
            GameScene::KamikazeDrone => write!(f, "Kamikaze Drone"),
            GameScene::Boid => write!(f, "Boid"),
            GameScene::Assets => write!(f, "Assets"),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Resource)]
pub struct Reload(Option<GameScene>);

fn update_scene(
    mut reload_scene: ResMut<Reload>,
    current_scene: Res<State<GameScene>>,
    mut next_scene: ResMut<NextState<GameScene>>,
    input_query: Query<&ActionState<InputAction>>,
) {
    if let Ok(input_action) = input_query.get_single() {
        for action in input_action.get_just_pressed() {
            if let InputAction::GameScene(mode) = action {
                next_scene.set(mode);
            }
        }
    }
}
