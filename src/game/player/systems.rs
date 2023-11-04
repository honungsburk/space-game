use std::collections::HashSet;

use super::components::{ContactForceInvulnerability, Player};
use super::{actions::*, components::DirectionControl};
use crate::game::average_velocity::AverageVelocity;
use crate::game::trauma::Trauma;
use crate::game::vitality::Health;
use crate::game::{assets::AssetDB, weapon::Weapon};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use leafwing_input_manager::prelude::*;

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
    mut commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&ActionState<PlayerAction>, &Transform, &mut Weapon), With<Player>>,
) {
    if let Ok((player_action_state, player_transform, mut weapon)) = query.get_single_mut() {
        if player_action_state.pressed(PlayerAction::FireWeapon) && weapon.can_fire() {
            let value = player_action_state.value(PlayerAction::FireWeapon);
            if value > 0.0 {
                weapon.fire(
                    &mut commands,
                    &asset_db,
                    &asset_server,
                    player_transform.clone(),
                );
            }
        }
    }
}

pub fn player_collision(
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut player_query: Query<
        (
            &mut Trauma,
            &mut Health,
            &ReadMassProperties,
            &mut ContactForceInvulnerability,
        ),
        With<Player>,
    >,
) {
    for contact_force_event in contact_force_events.iter() {
        if player_query.contains(contact_force_event.collider1)
            || player_query.contains(contact_force_event.collider2)
        {
            if let Ok((
                mut player_trauma,
                mut player_health,
                mass_properties,
                mut contact_force_invulnerability,
            )) = player_query.get_single_mut()
            {
                if !contact_force_invulnerability.is_invulnerable() {
                    // in the range 0-400
                    let adjusted_force =
                        contact_force_event.total_force_magnitude / mass_properties.0.mass;
                    let effect = (adjusted_force / 400.0).min(1.0);
                    // Take damage
                    player_health.take_damage_u32((effect * 10.0) as u32);
                    println!("effect: {}", effect);
                    // Trauma
                    player_trauma.add_trauma(effect);
                }
                contact_force_invulnerability.reset();
            }
        }
    }
}

pub fn update_contact_force_invulnerability(
    time: Res<Time>,
    mut player_query: Query<&mut ContactForceInvulnerability, With<Player>>,
) {
    for mut contact_force_invulnerability in player_query.iter_mut() {
        contact_force_invulnerability.tick(time.delta());
    }
}
