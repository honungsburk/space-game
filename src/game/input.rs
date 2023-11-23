//! All users input are mapped to Actions.
//!
//! Actions are defined in the defined where they are consumed. For example, the
//! `PlayerAction` is defined in the `Player` module.

use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use crate::scene::GameScene;

use super::{debug::DebugAction, player::PlayerShipAction};
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
    // CameraMovement(CameraMovementAction),
    // Ship Actions
    PlayerShip(PlayerShipAction),
    // Debug Actions
    Debug(DebugAction),
    // Game State Actions
    // GameScene(GameScene),
}

////////////////////////////////////////////////////////////////////////////////
/// Input Map
////////////////////////////////////////////////////////////////////////////////

pub fn create_input_map() -> InputMap<InputAction> {
    // Create an `InputMap` to add default inputs to
    let mut input_map: InputMap<InputAction> = InputMap::default();

    // Add Camera inputs
    // input_map.insert_multiple(vec![
    //     (
    //         InputKind::Keyboard(KeyCode::Up),
    //         InputAction::CameraMovement(CameraMovementAction::MoveUp),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::Down),
    //         InputAction::CameraMovement(CameraMovementAction::MoveDown),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::Left),
    //         InputAction::CameraMovement(CameraMovementAction::MoveLeft),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::Right),
    //         InputAction::CameraMovement(CameraMovementAction::MoveRight),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::W),
    //         InputAction::CameraMovement(CameraMovementAction::MoveUp),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::S),
    //         InputAction::CameraMovement(CameraMovementAction::MoveDown),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::A),
    //         InputAction::CameraMovement(CameraMovementAction::MoveLeft),
    //     ),
    //     (
    //         InputKind::Keyboard(KeyCode::D),
    //         InputAction::CameraMovement(CameraMovementAction::MoveRight),
    //     ),
    // ]);

    // Add PlayerShip inputs
    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::W),
            InputAction::PlayerShip(PlayerShipAction::ThrottleForward),
        ),
        (
            InputKind::Keyboard(KeyCode::S),
            InputAction::PlayerShip(PlayerShipAction::ThrottleBackwards),
        ),
        (
            InputKind::Keyboard(KeyCode::A),
            InputAction::PlayerShip(PlayerShipAction::RotateShipLeft),
        ),
        (
            InputKind::Keyboard(KeyCode::D),
            InputAction::PlayerShip(PlayerShipAction::RotateShipRight),
        ),
        (
            InputKind::Keyboard(KeyCode::L),
            InputAction::PlayerShip(PlayerShipAction::FireWeapon),
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::RightTrigger2),
            InputAction::PlayerShip(PlayerShipAction::ThrottleForward),
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::LeftTrigger2),
            InputAction::PlayerShip(PlayerShipAction::ThrottleBackwards),
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::South),
            InputAction::PlayerShip(PlayerShipAction::FireWeapon),
        ),
        (
            InputKind::DualAxis(DualAxis::left_stick()),
            InputAction::PlayerShip(PlayerShipAction::RotateShip),
        ),
    ]);

    // Add Debug inputs
    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::F1),
            InputAction::Debug(DebugAction::BackgroundGrid),
        ),
        (
            InputKind::Keyboard(KeyCode::F2),
            InputAction::Debug(DebugAction::CameraPosition),
        ),
        (
            InputKind::Keyboard(KeyCode::F3),
            InputAction::Debug(DebugAction::CameraSetpoint),
        ),
        (
            InputKind::Keyboard(KeyCode::F4),
            InputAction::Debug(DebugAction::Render),
        ),
        (
            InputKind::Keyboard(KeyCode::F5),
            InputAction::Debug(DebugAction::FPSCounter),
        ),
        (
            InputKind::Keyboard(KeyCode::F6),
            InputAction::Debug(DebugAction::VisionCone),
        ),
    ]);

    input_map.build()
}
