use bevy::{
    input::{gamepad::GamepadButtonType, keyboard::KeyCode},
    reflect::Reflect,
};
use leafwing_input_manager::{
    axislike::DualAxis, input_map::InputMap, user_input::InputKind, Actionlike,
};

/// Actions that can be performed by the player
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug, Reflect)]
pub enum PlayerShipAction {
    ThrottleForward,
    ThrottleBackwards,
    RotateShip,
    RotateShipLeft,
    RotateShipRight,
    FireWeapon,
}

/// Map inputs (keyboard/mouse/gamepad) to actions
pub fn create_input_map() -> InputMap<PlayerShipAction> {
    // Create an `InputMap` to add default inputs to
    let mut input_map: InputMap<PlayerShipAction> = InputMap::default();

    // Add PlayerShip inputs
    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::W),
            PlayerShipAction::ThrottleForward,
        ),
        (
            InputKind::Keyboard(KeyCode::S),
            PlayerShipAction::ThrottleBackwards,
        ),
        (
            InputKind::Keyboard(KeyCode::A),
            PlayerShipAction::RotateShipLeft,
        ),
        (
            InputKind::Keyboard(KeyCode::D),
            PlayerShipAction::RotateShipRight,
        ),
        (
            InputKind::Keyboard(KeyCode::L),
            PlayerShipAction::FireWeapon,
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::RightTrigger2),
            PlayerShipAction::ThrottleForward,
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::LeftTrigger2),
            PlayerShipAction::ThrottleBackwards,
        ),
        (
            InputKind::GamepadButton(GamepadButtonType::South),
            PlayerShipAction::FireWeapon,
        ),
        (
            InputKind::DualAxis(DualAxis::left_stick()),
            PlayerShipAction::RotateShip,
        ),
    ]);

    input_map.build()
}
