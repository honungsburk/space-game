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
    // Ship Actions
    PlayerShip(PlayerShipAction),
    // Debug Actions
    Debug(DebugAction),
    // Game State Actions
    GameScene(GameScene),
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

    // Add GameMode inputs
    // input_map.insert_chord([KeyCode::ControlLeft, KeyCode::R], InputAction::SceneReload);

    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::Key0),
            InputAction::GameScene(GameScene::None),
        ),
        (
            InputKind::Keyboard(KeyCode::Key1),
            InputAction::GameScene(GameScene::MainGame),
        ),
        (
            InputKind::Keyboard(KeyCode::Key2),
            InputAction::GameScene(GameScene::TurretPerformance),
        ),
        (
            InputKind::Keyboard(KeyCode::Key3),
            InputAction::GameScene(GameScene::PlayerDeath),
        ),
        (
            InputKind::Keyboard(KeyCode::Key4),
            InputAction::GameScene(GameScene::EnemyShipAI),
        ),
        (
            InputKind::Keyboard(KeyCode::Key5),
            InputAction::GameScene(GameScene::PlayerMovement),
        ),
        (
            InputKind::Keyboard(KeyCode::Key6),
            InputAction::GameScene(GameScene::Turret),
        ),
        (
            InputKind::Keyboard(KeyCode::Key7),
            InputAction::GameScene(GameScene::KamikazeDrone),
        ),
        (
            InputKind::Keyboard(KeyCode::Key8),
            InputAction::GameScene(GameScene::Boid),
        ),
        (
            InputKind::Keyboard(KeyCode::Key9),
            InputAction::GameScene(GameScene::Assets),
        ),
    ]);

    input_map.build()
}
