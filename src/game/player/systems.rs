use super::actions::*;
use super::components::Player;
use crate::game::weapon::Weapon;
use crate::game::{projectile as Projectile, weapon};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, transform::commands};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

impl PlayerAction {
    fn default_keyboard_mouse_input(action: PlayerAction) -> Result<UserInput, String> {
        // Match against the provided action to get the correct default keyboard-mouse input
        match action {
            Self::ThrottleForward => Ok(UserInput::Single(InputKind::Keyboard(KeyCode::W))),
            Self::ThrottleBackwards => Ok(UserInput::Single(InputKind::Keyboard(KeyCode::S))),
            Self::RotateShipLeft => Ok(UserInput::Single(InputKind::Keyboard(KeyCode::A))),
            Self::RotateShipRight => Ok(UserInput::Single(InputKind::Keyboard(KeyCode::D))),
            Self::FireWeapon => Ok(UserInput::Single(InputKind::Keyboard(KeyCode::L))),
            _ => Err(format!("No default keyboard-mouse input for {:?}", action)),
        }
    }

    fn default_gamepad_input(action: PlayerAction) -> Result<UserInput, String> {
        // Match against the provided action to get the correct default gamepad input
        match action {
            Self::ThrottleForward => Ok(UserInput::Single(InputKind::GamepadButton(
                GamepadButtonType::RightTrigger2,
            ))),
            Self::ThrottleBackwards => Ok(UserInput::Single(InputKind::GamepadButton(
                GamepadButtonType::LeftTrigger2,
            ))),
            Self::RotateShip => Ok(UserInput::Single(InputKind::DualAxis(
                DualAxis::left_stick(),
            ))),
            Self::FireWeapon => Ok(UserInput::Single(InputKind::GamepadButton(
                GamepadButtonType::South,
            ))),
            _ => Err(format!("No default gamepad-mouse input for {:?}", action)),
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    // Create an `InputMap` to add default inputs to
    let mut input_map = InputMap::default();

    // Loop through each action in `PlayerAction` and get the default `UserInput`,
    // then insert each default input into input_map
    for action in PlayerAction::variants() {
        if let Ok(keyboard_input) = PlayerAction::default_keyboard_mouse_input(action) {
            input_map.insert(keyboard_input, action);
        }
        if let Ok(gamepad_input) = PlayerAction::default_gamepad_input(action) {
            input_map.insert(gamepad_input, action);
        }
    }

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/playerShip1_blue.png"),
            ..default()
        })
        .insert(Player {})
        .insert(InputManagerBundle::<PlayerAction> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: input_map.build(),
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(Weapon::simple_laser());
}

pub fn control_ship(
    mut query: Query<(&mut ExternalImpulse, &ActionState<PlayerAction>, &Transform), With<Player>>,
) {
    if let Ok((mut player_impulse, player_action_state, player_transform)) = query.get_single_mut()
    {
        // player_impulse.impulse = Vec2::new(0.0, 0.0);
        // player_impulse.torque_impulse = 0.0;

        if player_action_state.pressed(PlayerAction::ThrottleForward) {
            // Note that some gamepad buttons are also tied to axes, so even though we used a
            // GamepadbuttonType::RightTrigger2 binding to trigger the throttle action, we can get a
            // variable value here if you have a variable right trigger on your gamepad.
            let value = player_action_state.value(PlayerAction::ThrottleForward);

            let impulse = player_transform
                .rotation
                .mul_vec3(Vec3::new(0.0, value * 0.001, 0.0));
            player_impulse.impulse = Vec2::new(impulse.x, impulse.y);
            // player_transform.rotation.into::<Vec2>() * Vec2::new(0.0, value * 0.001);
        }

        if player_action_state.pressed(PlayerAction::ThrottleBackwards) {
            // Note that some gamepad buttons are also tied to axes, so even though we used a
            // GamepadbuttonType::RightTrigger2 binding to trigger the throttle action, we can get a
            // variable value here if you have a variable right trigger on your gamepad.
            let value = player_action_state.value(PlayerAction::ThrottleBackwards);

            let impulse = player_transform
                .rotation
                .mul_vec3(Vec3::new(0.0, value * -0.0001, 0.0));
            player_impulse.impulse = Vec2::new(impulse.x, impulse.y);
        }

        if player_action_state.pressed(PlayerAction::RotateShip) {
            let value = player_action_state.value(PlayerAction::RotateShip);

            player_impulse.torque_impulse = value * 0.001;
        }

        if player_action_state.pressed(PlayerAction::RotateShipLeft) {
            let value = player_action_state.value(PlayerAction::RotateShipLeft);

            player_impulse.torque_impulse = value * 0.000000001;
        }
        if player_action_state.pressed(PlayerAction::RotateShipRight) {
            let value = player_action_state.value(PlayerAction::RotateShipRight);

            player_impulse.torque_impulse = value * -0.000000001;
        }
    }
}

pub fn fire_weapon(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&ActionState<PlayerAction>, &Transform, &mut Weapon), With<Player>>,
) {
    if let Ok((player_action_state, player_transform, mut weapon)) = query.get_single_mut() {
        if player_action_state.pressed(PlayerAction::FireWeapon) && weapon.can_fire() {
            let value = player_action_state.value(PlayerAction::FireWeapon);
            if value > 0.0 {
                weapon.fire(commands, &asset_server, player_transform.clone());
            }
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}
