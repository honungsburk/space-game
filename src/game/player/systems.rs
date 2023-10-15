use super::components::Player;
use super::{actions::*, components::DirectionControl};
use crate::game::average_velocity::AverageVelocity;
use crate::game::game_entity::GameEntityType;
use crate::game::trauma::Trauma;
use crate::game::{assets, assets::AssetDB, weapon::Weapon};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_db: Res<AssetDB>,
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
            texture: asset_server.load(asset_db.player_ship.sprite_path),
            ..default()
        })
        .insert(Player {})
        .insert(GameEntityType::Player)
        .insert(DirectionControl::default())
        .insert(InputManagerBundle::<PlayerAction> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: input_map.build(),
        })
        .insert(RigidBody::Dynamic)
        .insert(asset_db.player_ship.collider.clone())
        .insert(Trauma::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        // .insert(ContactForceEventThreshold(1.0))
        .insert(CollisionGroups::new(
            assets::PLAYER_GROUP.into(),
            assets::PLAYER_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            assets::PLAYER_GROUP.into(),
            assets::PLAYER_FILTER_MASK.into(),
        ))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        })
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(AverageVelocity::new(10))
        .insert(Weapon::simple_laser());
}

pub fn control_ship(
    mut query: Query<
        (
            &mut ExternalImpulse,
            &ActionState<PlayerAction>,
            &Transform,
            &mut DirectionControl,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut player_impulse, player_action_state, player_transform, mut direction_control)) =
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
                .mul_vec3(Vec3::new(0.0, value * 1.0, 0.0));
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
                .mul_vec3(Vec3::new(0.0, value * -0.2, 0.0));
            player_impulse.impulse = Vec2::new(impulse.x, impulse.y);
        }

        if player_action_state.pressed(PlayerAction::RotateShip) {
            if let Some(value) = player_action_state.clamped_axis_pair(PlayerAction::RotateShip) {
                let desired_direction = value.xy();

                if desired_direction.length() > 0.5 {
                    let setpoint = Vec2::Y.angle_between(desired_direction);
                    direction_control.set_setpoint(setpoint);
                    direction_control.turn_on();
                    // player_transform.rotation =
                    //     Quat::from_rotation_z(Vec2::Y.angle_between(desired_direction))
                }
            }
        }

        if player_action_state.pressed(PlayerAction::RotateShipLeft) {
            let value = player_action_state.value(PlayerAction::RotateShipLeft);

            player_impulse.torque_impulse = value * 0.005;
            direction_control.turn_off();
        }
        if player_action_state.pressed(PlayerAction::RotateShipRight) {
            let value = player_action_state.value(PlayerAction::RotateShipRight);

            player_impulse.torque_impulse = value * -0.005;
            direction_control.turn_off();
        }
    }
}

pub fn update_player_rotation(
    time: Res<Time>,
    mut query: Query<(&mut ExternalImpulse, &Transform, &mut DirectionControl), With<Player>>,
) {
    for (mut player_impulse, player_transform, mut direction_control) in query.iter_mut() {
        let (_, _, current_angle) = player_transform.rotation.to_euler(EulerRot::XYZ);

        if let Some(control_signal) = direction_control.update(current_angle, time.delta_seconds())
        {
            player_impulse.torque_impulse = control_signal * 0.005;
        }
    }
}

pub fn fire_weapon(
    commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&ActionState<PlayerAction>, &Transform, &mut Weapon), With<Player>>,
) {
    if let Ok((player_action_state, player_transform, mut weapon)) = query.get_single_mut() {
        if player_action_state.pressed(PlayerAction::FireWeapon) && weapon.can_fire() {
            let value = player_action_state.value(PlayerAction::FireWeapon);
            if value > 0.0 {
                weapon.fire(commands, &asset_db, &asset_server, player_transform.clone());
            }
        }
    }
}

// CollisionEvent::Started(entity1, entity2, _) => {

pub fn player_collision(
    mut collison_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Trauma, &AverageVelocity, &Velocity), With<Player>>,
) {
    for collision_event in collison_events.iter() {
        // TODO: Scale trauma based on force applied to the player
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if player_query.contains(*entity1) || player_query.contains(*entity2) {
                    if let Ok((mut player_trauma, average_velocity, velocity)) =
                        player_query.get_single_mut()
                    {
                        // TODO: We need a MAX_VELOCITY to scale this by
                        player_trauma.add_trauma(
                            (average_velocity.get_linvel() - velocity.linvel).length() / 200.0,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}
