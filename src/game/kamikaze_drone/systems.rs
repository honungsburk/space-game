use crate::misc::rapier_extension;

use super::components::{BoidTargets, KamikazeDroneLabel, KamikazeDroneSensorLabel};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::{CollisionEvent, QueryFilter, RapierContext, Velocity};

const KAMIKAZE_DRONE_MAX_SPEED: f32 = 200.0;
const KAMIKAZE_DRONE_MIN_SPEED: f32 = 20.0;

// // boids: https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html
pub fn update_kamikaze_drone(
    gizmos: Gizmos,
    rapier_ctx: Res<RapierContext>,
    mut kamikaze_query: Query<
        (Entity, &mut Transform, &BoidTargets, &mut Velocity),
        With<KamikazeDroneLabel>,
    >,
) {
    let filter = QueryFilter::new().exclude_sensors();
    let mut giz = Some(gizmos);

    // We muse use unsafe here because we are iterating over the same query twice. However, we know
    // that we will never have two references to the same entity at the same time, so this is safe.

    let mut compute_table = HashMap::<Entity, (Vec2, Quat)>::default();

    for (entity, kamikaze_transform, boid_targets, kamikaze_velocity) in kamikaze_query.iter() {
        // If the path is unobstructed, move as boid
        let avoid_radius = 10.0;
        let avoid_factor = 1.0;
        let matching_factor = 1.0;
        let centering_factor = 1.0;

        let mut boid_pos = Vec2::ZERO;
        let mut boid_close = Vec2::ZERO;
        let mut boid_match_velocity = Vec2::ZERO;
        let mut neighboring_boids = 0.0;

        for target in boid_targets.0.iter() {
            // Immutable borrow: kamikaze_query.get(*target)
            if let Ok((_, other_transform, _, other_velocity)) = kamikaze_query.get(*target) {
                let diff =
                    (kamikaze_transform.translation - other_transform.translation).truncate();

                if diff.length() < avoid_radius {
                    // Seperation
                    boid_close += diff;
                    boid_pos += other_transform.translation.truncate();
                } else {
                    // Alignment
                    boid_match_velocity += other_velocity.linvel;
                    neighboring_boids += 1.0;
                }
            }
        }

        let mut kamikaze_new_velocity = boid_close * avoid_factor;

        if neighboring_boids > 0.0 {
            kamikaze_new_velocity += (boid_match_velocity / neighboring_boids
                - kamikaze_velocity.linvel)
                * matching_factor;

            kamikaze_new_velocity += (boid_pos / neighboring_boids
                - kamikaze_transform.translation.truncate())
                * centering_factor;
        }
        let speed = kamikaze_new_velocity.length();

        let mut new_velocity = kamikaze_new_velocity;

        if speed == 0.0 {
            new_velocity = Vec2::Y * KAMIKAZE_DRONE_MAX_SPEED;
        } else if speed > KAMIKAZE_DRONE_MAX_SPEED {
            new_velocity = kamikaze_new_velocity.normalize_or_zero() * KAMIKAZE_DRONE_MAX_SPEED;
        } else if speed < KAMIKAZE_DRONE_MIN_SPEED {
            new_velocity = kamikaze_new_velocity.normalize_or_zero() * KAMIKAZE_DRONE_MIN_SPEED;
        }

        let new_angle = Vec2::Y.angle_between(new_velocity);
        let mut new_rotation = Quat::from_rotation_z(new_angle);

        let check_transform = *kamikaze_transform * Transform::from_rotation(new_rotation);

        if let Some(path) = rapier_extension::find_unobstructed_path(
            &rapier_ctx,
            &mut giz,
            &check_transform,
            1.0,
            25.0,
            200.0,
            20.0,
            filter,
            None,
        ) {
            // if the unobstructed path is not the same as the current path, update the path
            if path.angle_between(new_velocity).abs() > 0.0 {
                new_velocity = path * KAMIKAZE_DRONE_MAX_SPEED;
                new_rotation = Quat::from_rotation_z(Vec2::Y.angle_between(path));
            }
            compute_table.insert(entity, (new_velocity, new_rotation));
        }
    }

    for (entity, (new_velocity, new_rotation)) in compute_table.iter() {
        if let Ok((_, mut kamikaze_transform, _, mut kamikaze_velocity)) =
            kamikaze_query.get_mut(*entity)
        {
            kamikaze_transform.rotation = *new_rotation;
            kamikaze_velocity.linvel = *new_velocity;
        }
    }
}

pub fn update_kamikaze_drone_targets(
    mut collision_events: EventReader<CollisionEvent>,
    mut kamikaze_drone_query: Query<&mut BoidTargets, With<KamikazeDroneLabel>>,
    sensor_query: Query<&Parent, With<KamikazeDroneSensorLabel>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let sensor = sensor_query.get(*entity1).or(sensor_query.get(*entity2));
                if let Ok(parent) = sensor {
                    let other_entity = if kamikaze_drone_query.contains(*entity1) {
                        *entity1
                    } else {
                        *entity2
                    };

                    let parent_entity = parent.get();

                    // Avoid adding self as a target
                    // Program will crash if we don't do this
                    if other_entity == parent_entity {
                        continue;
                    }

                    if let Ok(mut targets) = kamikaze_drone_query.get_mut(parent_entity) {
                        targets.0.insert(other_entity);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                let sensor = sensor_query.get(*entity1).or(sensor_query.get(*entity2));
                if let Ok(parent) = sensor {
                    let other_entity = if kamikaze_drone_query.contains(*entity1) {
                        *entity1
                    } else {
                        *entity2
                    };

                    if let Ok(mut targets) = kamikaze_drone_query.get_mut(parent.get()) {
                        targets.0.remove(&other_entity);
                    }
                }
            }
        }
    }
}
