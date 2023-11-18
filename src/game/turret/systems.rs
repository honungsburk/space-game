use super::ai::TurretAI;
use super::{ai, components::*};
use crate::game::assets::AssetDB;
use crate::game::targets::Targets;
use crate::game::{player::components::Player, targets::Target, weapon::Weapon};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{
    geometry::*,
    prelude::{CollisionEvent, ExternalImpulse, Velocity},
};

pub fn update_ai(mut query: Query<(&mut ai::TurretAI, &Targets)>, time: Res<Time>) {
    for (mut turret_ai, targets) in query.iter_mut() {
        turret_ai.state.update(&time, !targets.is_empty());
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
        &Targets,
        &mut ExternalImpulse,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    if dt == 0.0 {
        return;
    }
    for (turret_ai, mut rotation_control, turret_global_transform, targets, mut turret_impulse) in
        query.iter_mut()
    {
        if !turret_ai.state.is_targeting() {
            continue;
        }

        if let Some(target) = targets.current_target() {
            let desired_angel =
                Vec2::Y.angle_between(target.location - turret_global_transform.translation.xy());

            // if target.location - turret_transform.translation().xy() == Vec2::ZERO then desired_angel is NaN
            if desired_angel.is_nan() {
                continue;
            }

            rotation_control.control.set_setpoint(desired_angel);

            let (_, _, current_angle) = turret_global_transform.rotation.to_euler(EulerRot::XYZ);

            let control_signal = rotation_control.control.update(current_angle, dt);

            turret_impulse.torque_impulse = control_signal * 0.001;
        }
    }
}

pub fn update_turret_target(
    mut target_query: Query<&mut Targets>,
    transform_query: Query<&GlobalTransform>,
) {
    for mut targets in target_query.iter_mut() {
        targets.for_each(|target| {
            if let Ok(target_transform) = transform_query.get(target.entity) {
                target.location = target_transform.translation().xy();
            }
        })
    }
}

// TODO: Create a custom event for this CollisionEvent => CustomEvent
// Then we only need to read through the events once, but will we be delayed one frame?
pub fn register_turret_target(
    mut collision_events: EventReader<CollisionEvent>,
    mut targets_query: Query<&mut Targets, Without<Player>>,
    sensor_query: Query<(&Parent, &Sensor)>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let sensor = sensor_query.get(*entity1).or(sensor_query.get(*entity2));

                if let Ok((parent, _)) = sensor {
                    let targets = targets_query.get_mut(parent.get());
                    let player_entity = if player_query.contains(*entity1) {
                        *entity1
                    } else {
                        *entity2
                    };

                    let player = player_query.get(player_entity);

                    if let (Ok(mut targets), Ok(player_global)) = (targets, player) {
                        let player_location = player_global.translation().xy();

                        let target = Target {
                            entity: player_entity,
                            location: player_location,
                        };

                        targets.add(target);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                let sensor = sensor_query.get(*entity1).or(sensor_query.get(*entity2));
                if let Ok((parent, _)) = sensor {
                    let targets = targets_query.get_mut(parent.get());
                    if let Ok(mut targets) = targets {
                        if player_query.contains(*entity1) {
                            targets.remove(*entity1);
                        } else if player_query.contains(*entity2) {
                            targets.remove(*entity2);
                        }
                    }
                }
            }
        }
    }
}

pub fn get_target<'a>(
    targets_query: &'a mut Query<&mut Targets, Without<Player>>,
    entity1: &Entity,
    entity2: &Entity,
) -> Option<Mut<'a, Targets>> {
    if targets_query.contains(*entity1) {
        return targets_query.get_mut(*entity1).ok();
    } else if targets_query.contains(*entity2) {
        return targets_query.get_mut(*entity2).ok();
    } else {
        return None;
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
    mut turret_query: Query<&mut Targets, With<TurretLabel>>,
    mut turret_radius_query: Query<(&Parent, &mut Stroke), With<TurretSensorLabel>>,
) {
    for (parent, mut stroke) in turret_radius_query.iter_mut() {
        if let Ok(mut targets) = turret_query.get_mut(parent.get()) {
            if targets.has_changed() {
                if targets.is_empty() {
                    stroke.color = Color::rgba(0.0, 0.0, 0.0, 0.2);
                } else {
                    stroke.color = Color::rgba(1.0, 0.0, 0.0, 0.4);
                }
            }
        }
    }
}
