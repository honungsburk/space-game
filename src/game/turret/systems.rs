use super::ai::TurretAI;
use super::{ai, components::*};
use crate::game::assets::AssetDB;
use crate::game::sensor::SensorTargetVec2;
use crate::game::weapon::Weapon;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};

pub fn update_ai(
    mut turret_query: Query<&mut ai::TurretAI>,
    sensor_query: Query<(&Parent, &SensorTargetVec2)>,
    time: Res<Time>,
) {
    for (parent, target) in sensor_query.iter() {
        if let Ok(mut turret_ai) = turret_query.get_mut(parent.get()) {
            turret_ai.state.update(&time, target.has_target());
        }
    }
}

pub fn fire_weapon(
    mut query: Query<(&ai::TurretAI, &mut Weapon, &Transform)>,
    mut commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
) {
    for (turret_ai, mut weapon, transform) in query.iter_mut() {
        if turret_ai.state.is_firing() {
            weapon.fire(&mut commands, &asset_db, &asset_server, *transform)
        }
    }
}

pub fn update_turret_rotation(
    mut query: Query<(
        &TurretAI,
        &mut RotationControl,
        &Transform,
        &Children,
        &mut ExternalImpulse,
    )>,
    target_query: Query<&SensorTargetVec2>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    if dt == 0.0 {
        return;
    }
    for (turret_ai, mut rotation_control, turret_global_transform, children, mut turret_impulse) in
        query.iter_mut()
    {
        if !turret_ai.state.is_targeting() {
            continue;
        }

        // If we have a target, set the desired angle to the angle between the turret and the target
        for child in children.iter() {
            if let Ok(Some((_, location))) = target_query.get(*child).map(|t| t.get()) {
                let desired_angel =
                    Vec2::Y.angle_between(*location - turret_global_transform.translation.xy());

                if desired_angel.is_nan() {
                    continue;
                }

                rotation_control.control.set_setpoint(desired_angel);
            }
        }

        // Rotate the turret towards the desired angle
        let (_, _, current_angle) = turret_global_transform.rotation.to_euler(EulerRot::XYZ);

        let control_signal = rotation_control.control.update(current_angle, dt);

        turret_impulse.torque_impulse = control_signal * 0.001;
    }
}

pub fn update_stationary_control(
    mut query: Query<(&mut StationaryControl, &Velocity, &mut ExternalImpulse)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    if dt == 0.0 {
        return;
    }

    for (mut stationary_control, turret_velocity, mut turret_impulse) in query.iter_mut() {
        if turret_velocity.linvel.length() == 0.0 {
            continue;
        }

        let control_signal = stationary_control
            .control
            .update(turret_velocity.linvel, dt);

        let new_impulse = (control_signal * 1.0).clamp_length_max(0.4);
        if new_impulse.length() > 0.0 {
            turret_impulse.impulse = new_impulse
            //TODO: add max impulse
        }
    }
}

// WARNING: You must perform change detection if you are going to use this system
// the lyon plugin will check if a shape has changed, and if it has it will update the mesh
// This is very expensive, so we only want to do it when we need to.
pub fn update_turret_radius_outline(
    mut turret_radius_query: Query<(&mut Stroke, &mut SensorTargetVec2), With<TurretSensorLabel>>,
) {
    for (mut stroke, mut target) in turret_radius_query.iter_mut() {
        if target.has_changed() {
            if target.has_target() {
                stroke.color = Color::rgba(1.0, 0.0, 0.0, 0.4);
            } else {
                stroke.color = Color::rgba(0.0, 0.0, 0.0, 0.2);
            }
        }
    }
}
