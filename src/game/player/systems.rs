use super::components::{ContactForceInvulnerability, Player};
use super::PlayerShipAction;
use crate::game::control_system::DirectionControl;
use crate::game::trauma::Trauma;
use crate::game::vitality::Health;
use crate::game::weapon::Weapon;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn control_ship(
    mut query: Query<
        (
            &mut ExternalImpulse,
            &Transform,
            &mut DirectionControl,
            &ActionState<PlayerShipAction>,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut player_impulse, player_transform, mut direction_control, input_action)) =
        query.get_single_mut()
    {
        if input_action.pressed(PlayerShipAction::ThrottleForward) {
            // Note that some gamepad buttons are also tied to axes, so even though we used a
            // GamepadbuttonType::RightTrigger2 binding to trigger the throttle action, we can get a
            // variable value here if you have a variable right trigger on your gamepad.
            // we expect a value between 0.0 and 1.0
            let value: f32 = input_action
                .value(PlayerShipAction::ThrottleForward)
                .clamp(0.0, 1.0);

            let impulse = player_transform
                .rotation
                .mul_vec3(Vec3::new(0.0, value, 0.0));
            player_impulse.impulse = Vec2::new(impulse.x, impulse.y);
            // player_transform.rotation.into::<Vec2>() * Vec2::new(0.0, value * 0.001);
        }

        if input_action.pressed(PlayerShipAction::ThrottleBackwards) {
            // Note that some gamepad buttons are also tied to axes, so even though we used a
            // GamepadbuttonType::RightTrigger2 binding to trigger the throttle action, we can get a
            // variable value here if you have a variable right trigger on your gamepad.
            let value = input_action.value(PlayerShipAction::ThrottleBackwards);

            let impulse = player_transform
                .rotation
                .mul_vec3(Vec3::new(0.0, value * -0.2, 0.0));
            player_impulse.impulse = Vec2::new(impulse.x, impulse.y);
        }

        if input_action.pressed(PlayerShipAction::RotateShip) {
            if let Some(value) = input_action.clamped_axis_pair(PlayerShipAction::RotateShip) {
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

        if input_action.pressed(PlayerShipAction::RotateShipLeft) {
            let value = input_action.value(PlayerShipAction::RotateShipLeft);

            player_impulse.torque_impulse = value * 0.005;
            direction_control.turn_off();
        }
        if input_action.pressed(PlayerShipAction::RotateShipRight) {
            let value = input_action.value(PlayerShipAction::RotateShipRight);

            player_impulse.torque_impulse = value * -0.005;
            direction_control.turn_off();
        }
    }
}

pub fn fire_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Transform, &mut Weapon, &ActionState<PlayerShipAction>), With<Player>>,
) {
    if let Ok((player_transform, mut weapon, action)) = query.get_single_mut() {
        if action.pressed(PlayerShipAction::FireWeapon) && weapon.can_fire() {
            let value = action.value(PlayerShipAction::FireWeapon);
            if value > 0.0 {
                weapon.fire(&mut commands, &asset_server, player_transform.clone());
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
    for contact_force_event in contact_force_events.read() {
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
                        contact_force_event.total_force_magnitude / mass_properties.mass;
                    let effect = (adjusted_force / 400.0).min(1.0);
                    // Take damage
                    player_health.take_damage_u32((effect * 10.0) as u32);
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
