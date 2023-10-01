use super::actions::*;
use super::components::Player;
use crate::game::weapon::Weapon;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    // Create an `InputMap` to add default inputs to
    let mut input_map = InputMap::default()
        .insert(
            InputKind::Keyboard(KeyCode::W),
            PlayerAction::ThrottleForward,
        )
        .insert(
            InputKind::Keyboard(KeyCode::S),
            PlayerAction::ThrottleBackwards,
        )
        .insert(
            InputKind::Keyboard(KeyCode::A),
            PlayerAction::RotateShipLeft,
        )
        .insert(
            InputKind::Keyboard(KeyCode::D),
            PlayerAction::RotateShipRight,
        )
        .insert(InputKind::Keyboard(KeyCode::L), PlayerAction::FireWeapon)
        .insert(
            InputKind::GamepadButton(GamepadButtonType::RightTrigger2),
            PlayerAction::ThrottleForward,
        )
        .insert(
            InputKind::GamepadButton(GamepadButtonType::LeftTrigger2),
            PlayerAction::ThrottleBackwards,
        )
        .insert(
            InputKind::GamepadButton(GamepadButtonType::South),
            PlayerAction::FireWeapon,
        )
        .insert(
            InputKind::DualAxis(DualAxis::left_stick()),
            PlayerAction::RotateShip,
        )
        .build();

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
    mut query: Query<
        (
            &mut ExternalImpulse,
            &ActionState<PlayerAction>,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut player_impulse, player_action_state, mut player_transform)) =
        query.get_single_mut()
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
            if let Some(value) = player_action_state.clamped_axis_pair(PlayerAction::RotateShip) {
                let desired_direction = value.xy();

                if desired_direction.length() > 0.5 {
                    player_transform.rotation =
                        Quat::from_rotation_z(Vec2::Y.angle_between(desired_direction))
                }
            }
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
