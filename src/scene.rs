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
use leafwing_input_manager::{
    input_map::InputMap, plugin::InputManagerPlugin, prelude::ActionState, user_input::InputKind,
    Actionlike, InputManagerBundle,
};
use std::fmt;

/// Add to entities that are part of the scene, so they can be
/// easily despawned when the scene is exited.
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct SceneEntityLabel;

impl SceneEntityLabel {
    pub fn despawn(mut commands: Commands, query: Query<Entity, With<SceneEntityLabel>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameScene>()
            .init_resource::<Reload>()
            .add_plugins(InputManagerPlugin::<GameScene>::default())
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
            .add_systems(Startup, create)
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States, Reflect, Actionlike)]
pub enum GameScene {
    // Real Game Modes
    None, // No mode
    MainGame,
    // Debug Game Modes
    #[default]
    TurretPerformance, // Performance testing mode with a lot of turrets
    PlayerDeath, // Player death testing mode
    EnemyShipAI,
    PlayerMovement,
    Turret,
    KamikazeDrone,
    Boid,
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

fn create(mut commands: Commands) {
    commands.spawn(InputManagerBundle {
        action_state: ActionState::default(),
        input_map: create_input_map(),
    });
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Resource)]
pub struct Reload(Option<GameScene>);

fn update_scene(
    mut next_scene: ResMut<NextState<GameScene>>,
    input_query: Query<&ActionState<GameScene>>,
) {
    if let Ok(input_action) = input_query.get_single() {
        for action in input_action.get_just_pressed() {
            next_scene.set(action);
        }
    }
}

fn create_input_map() -> InputMap<GameScene> {
    let mut input_map: InputMap<GameScene> = InputMap::default();

    // Add Camera inputs
    input_map.insert_multiple(vec![
        (InputKind::Keyboard(KeyCode::Key0), GameScene::None),
        (InputKind::Keyboard(KeyCode::Key1), GameScene::MainGame),
        (
            InputKind::Keyboard(KeyCode::Key2),
            GameScene::TurretPerformance,
        ),
        (InputKind::Keyboard(KeyCode::Key3), GameScene::PlayerDeath),
        (InputKind::Keyboard(KeyCode::Key4), GameScene::EnemyShipAI),
        (
            InputKind::Keyboard(KeyCode::Key5),
            GameScene::PlayerMovement,
        ),
        (InputKind::Keyboard(KeyCode::Key6), GameScene::Turret),
        (InputKind::Keyboard(KeyCode::Key7), GameScene::KamikazeDrone),
        (InputKind::Keyboard(KeyCode::Key8), GameScene::Boid),
        (InputKind::Keyboard(KeyCode::Key9), GameScene::Assets),
    ]);

    input_map
}
