use std::{cell::RefCell, collections::HashMap};

use crate::game::{assets::groups, guard_point::GuardPoint};

use super::components::{KamikazeDroneLabel, KamikazeDroneTargetLabel};
use bevy::prelude::*;
use bevy_rapier2d::{
    geometry::CollisionGroups, pipeline::QueryFilter, plugin::RapierContext, prelude::Velocity,
};

const DRONE_MAX_SPEED: f32 = 200.0;
const DRONE_MIN_SPEED: f32 = 20.0;

const DRONE_MAX_Y: f32 = 200.0;

const FOLLOW_RADIUS: f32 = 200.0;
const AVOID_RADIUS: f32 = 40.0;
const AVOID_FACTOR: f32 = 0.05;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FACTOR: f32 = 0.0005;

const TARGET_RADIUS: f32 = 300.0;

const COLLISION_AVODIANCE_FACTOR: f32 = 0.1;

#[derive(Debug, PartialEq, Default)]
struct UpdateCompute {
    close: Vec2,
    velocity_sum: Vec2,
    position_sum: Vec2,
    neighbors: f32,
}

// drones: https://vanhunteradams.com/Pico/Animal_Movement/drones-algorithm.html
pub fn update(
    mut gizmos: Gizmos,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    targets_query: Query<&Transform, (With<KamikazeDroneTargetLabel>, Without<KamikazeDroneLabel>)>,
    mut drone_query: Query<
        (Entity, &mut Transform, &mut Velocity, Option<&GuardPoint>),
        (With<KamikazeDroneLabel>, Without<KamikazeDroneTargetLabel>),
    >,
) {
    let mut compute_table = HashMap::<Entity, RefCell<UpdateCompute>>::default();

    // Apply the computation
    for [drone_1, drone_2] in drone_query.iter_combinations() {
        let diff = (drone_1.1.translation - drone_2.1.translation).truncate();
        let distance = diff.length();

        // This makes sure all entities exist in compute table
        let (compute1_cell, compute2_cell) =
            get_compute(&drone_1.0, &drone_2.0, &mut compute_table);
        if distance < FOLLOW_RADIUS {
            let compute1 = &mut *compute1_cell.borrow_mut();
            let compute2 = &mut *compute2_cell.borrow_mut();

            if distance < AVOID_RADIUS {
                // Seperation
                compute1.close += diff;
                compute2.close -= diff;
            } else {
                // Alignment
                compute1.position_sum += drone_2.1.translation.truncate();
                compute2.position_sum += drone_1.1.translation.truncate();
                compute1.velocity_sum += drone_2.2.linvel;
                compute2.velocity_sum += drone_1.2.linvel;
                compute1.neighbors += 1.0;
                compute2.neighbors += 1.0;
            }
        }
    }

    // let shape = KAMIKAZE_DRONE.collider();

    // Apply the computation
    for (entity, mut t, mut v, guard_point_opt) in drone_query.iter_mut() {
        if let Some(compute_cell) = compute_table.get(&entity) {
            let compute = compute_cell.borrow();

            let mut velocity_change = compute.close * AVOID_FACTOR;

            if compute.neighbors > 0.0 {
                velocity_change +=
                    ((compute.velocity_sum / compute.neighbors) - v.linvel) * MATCHING_FACTOR;

                velocity_change += ((compute.position_sum / compute.neighbors)
                    - t.translation.truncate())
                    * CENTERING_FACTOR;
            }

            v.linvel += velocity_change;
        }

        for target_t in targets_query.iter() {
            let diff = target_t.translation.truncate() - t.translation.truncate();
            let distance = diff.length();

            if distance < TARGET_RADIUS {
                let speed = v.linvel.length();
                v.linvel += speed * diff.normalize() * ((0.5 * distance / TARGET_RADIUS) + 0.5);
            }
        }

        if let Some(guard_point) = guard_point_opt {
            let diff = guard_point.point - t.translation.truncate();
            // When we are 90% of the way to the guard point, we start trying to turn around
            let distance = diff.length() - guard_point.max_distance * 0.9;

            if distance > 0.0 {
                let speed = v.linvel.length();

                // v_dist will be 1.0 when 95% of the way to the guard point, and 2.0 when 100% of the way, and 3.0 when 105% of the way ...
                let distance_strength = guard_point.max_distance * 0.05;
                let v_dist = distance / distance_strength;

                v.linvel += speed * diff.normalize() * v_dist * v_dist;
            }
        }

        // Check if the drone is heading to words collision

        let ray_start = t.translation.truncate();
        let mut filter = QueryFilter::default(); // Should only look for meteors?
        filter = filter.exclude_sensors();
        filter = filter.groups(CollisionGroups {
            memberships: groups::ENEMY_GROUP,
            filters: groups::METEOR_GROUP & groups::ENEMY_GROUP,
        });

        // Cast a ray one second into the future fo the current direction
        // Note: Toi can be zero if the ray starts inside a collider. In that case, we ignore the ray.

        // First check if the ray hits a collider, in which case we need to avoid it
        if let Some((entity, ray_intersection)) =
            rapier_context.cast_ray_and_get_normal(ray_start, v.linvel, 1.0, true, filter)
        {
            // We filter out the drones, because we don't want them to avoid each other
            if ray_intersection.toi > 0.0 && !compute_table.contains_key(&entity) {
                let avoidance = v.linvel.length()
                    * ray_intersection.normal
                    * COLLISION_AVODIANCE_FACTOR
                    * (1.0 / ray_intersection.toi - 1.0);
                v.linvel += avoidance;

                //direction * COLLISION_AVODIANCE_FACTOR * (1.0 / toi);
                gizmos.line(
                    ray_start.extend(0.0),
                    (ray_start + v.linvel * ray_intersection.toi).extend(0.0),
                    Color::RED,
                );
            }
        }

        // if let Some((_, toi)) =
        //     rapier_context.cast_shape(ray_start, 0.0, v.linvel, &shape, 1.0, true, filter)
        // {
        //     // let details: ToiDetails = ray_intersection.details.unwrap();

        //     if toi.toi > 0.0 {
        //         //
        //         let avoidance =
        //             v.linvel.length() * COLLISION_AVODIANCE_FACTOR * (1.0 / toi.toi - 1.0);
        //         v.linvel -= avoidance;

        //         //direction * COLLISION_AVODIANCE_FACTOR * (1.0 / toi);
        //         gizmos.line(
        //             ray_start.extend(0.0),
        //             (ray_start + v.linvel * toi.toi).extend(0.0),
        //             Color::RED,
        //         );
        //     } else {
        //         // if the toi is zero, the ray starts inside the collider, so we will turn towards the center of the world
        //         v.linvel = (Vec2::ZERO - ray_start).normalize() * drone_MAX_SPEED;
        //     }
        // }

        if v.linvel.length() == 0.0 {
            v.linvel = Vec2::Y * DRONE_MAX_SPEED;
        }
        v.linvel.clamp_length(DRONE_MIN_SPEED, DRONE_MAX_SPEED);

        // Update position
        // TODO: You need the average velocity of the frame, and delta_second is the velocity of the last frame, so this is wrong
        t.translation += v.linvel.extend(0.0) * time.delta_seconds();

        let new_rotation = Quat::from_rotation_z(Vec2::Y.angle_between(v.linvel));

        if new_rotation.angle_between(t.rotation) < 0.1 {
            v.linvel = (v.linvel * 1.1).clamp_length(DRONE_MIN_SPEED, DRONE_MAX_SPEED)
        }

        t.rotation = new_rotation;
    }
}

// This is wrong right? The the returned reference is only valid until insert is called on the HashMap...
// Or... The values live... Like you can't remove the entity from the HashMap without dropping the value...
fn get_compute<'a>(
    entity1: &Entity,
    entity2: &Entity,
    compute_table: &'a mut HashMap<Entity, RefCell<UpdateCompute>>,
) -> (&'a RefCell<UpdateCompute>, &'a RefCell<UpdateCompute>) {
    if !compute_table.contains_key(entity1) {
        compute_table.insert(*entity1, RefCell::new(UpdateCompute::default()));
    }

    if !compute_table.contains_key(entity2) {
        compute_table.insert(*entity2, RefCell::new(UpdateCompute::default()));
    }
    return (
        compute_table.get(entity1).unwrap(),
        compute_table.get(entity2).unwrap(),
    );
}
