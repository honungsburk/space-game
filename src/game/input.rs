//! All users input are mapped to Actions.
//!
//! Actions are defined in the defined where they are consumed. For example, the
//! `PlayerAction` is defined in the `Player` module.

use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};
// Actions

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            .add_systems(Startup, spawn_input);
    }
}

#[derive(Component)]
pub struct InputLabel;

pub fn spawn_input(mut commands: Commands) {
    commands
        .spawn(InputLabel)
        .insert(InputManagerBundle::<InputAction> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from user input to actions
            input_map: create_input_map(),
        });
}

////////////////////////////////////////////////////////////////////////////////
/// Actions
////////////////////////////////////////////////////////////////////////////////

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug, Reflect)]
pub enum InputAction {
    // Player Actions
    PlayerThrottleForward,
    PlayerThrottleBackwards,
    PlayerRotateShip,
    PlayerRotateShipLeft,
    PlayerRotateShipRight,
    PlayerFireWeapon,
    // Debug Actions
    DebugBackgroundGrid,
    DebugCameraPosition,
    DebugCameraSetpoint,
    DebugRender,
    DebugFPSCounter,
    DebugVisionCone,
    // Game State Actions
    SceneReload,
    SceneNone,
    SceneMainGame,
    SceneTurretPerformance,
    ScenePlayerDeath,
    SceneEnemyShipAI,
    ScenePlayerMovement,
    SceneTurret,
}

////////////////////////////////////////////////////////////////////////////////
/// Input Map
////////////////////////////////////////////////////////////////////////////////

pub fn create_input_map() -> InputMap<InputAction> {
    // Create an `InputMap` to add default inputs to
    let mut input_map: InputMap<InputAction> = InputMap::default();

    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::W),
            InputAction::PlayerThrottleForward,
        ),
        (
            InputKind::Keyboard(KeyCode::S),
            InputAction::PlayerThrottleBackwards,
        ),
        (
            InputKind::Keyboard(KeyCode::A),
            InputAction::PlayerRotateShipLeft,
        ),
        (
            InputKind::Keyboard(KeyCode::D),
            InputAction::PlayerRotateShipRight,
        ),
        (
            InputKind::Keyboard(KeyCode::L),
            InputAction::PlayerFireWeapon,
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::RightTrigger2),
            InputAction::PlayerThrottleForward,
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::LeftTrigger2),
            InputAction::PlayerThrottleBackwards,
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::South),
            InputAction::PlayerFireWeapon,
        ),
        (
            InputKind::DualAxis(DualAxis::left_stick()),
            InputAction::PlayerRotateShip,
        ),
    ]);

    // Add Debug inputs
    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::F1),
            InputAction::DebugBackgroundGrid,
        ),
        (
            InputKind::Keyboard(KeyCode::F2),
            InputAction::DebugCameraPosition,
        ),
        (
            InputKind::Keyboard(KeyCode::F3),
            InputAction::DebugCameraSetpoint,
        ),
        (InputKind::Keyboard(KeyCode::F4), InputAction::DebugRender),
        (
            InputKind::Keyboard(KeyCode::F5),
            InputAction::DebugFPSCounter,
        ),
        (
            InputKind::Keyboard(KeyCode::F6),
            InputAction::DebugVisionCone,
        ),
    ]);

    // Add GameMode inputs
    input_map.insert_chord([KeyCode::ControlLeft, KeyCode::R], InputAction::SceneReload);

    input_map.insert_multiple(vec![
        (InputKind::Keyboard(KeyCode::Key0), InputAction::SceneNone),
        (
            InputKind::Keyboard(KeyCode::Key1),
            InputAction::SceneMainGame,
        ),
        (
            InputKind::Keyboard(KeyCode::Key2),
            InputAction::SceneTurretPerformance,
        ),
        (
            InputKind::Keyboard(KeyCode::Key3),
            InputAction::ScenePlayerDeath,
        ),
        (
            InputKind::Keyboard(KeyCode::Key4),
            InputAction::SceneEnemyShipAI,
        ),
        (
            InputKind::Keyboard(KeyCode::Key5),
            InputAction::ScenePlayerMovement,
        ),
        (InputKind::Keyboard(KeyCode::Key6), InputAction::SceneTurret),
    ]);

    input_map.build()
}
