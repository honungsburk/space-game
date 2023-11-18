use std::{cell::RefCell, collections::HashMap};

use super::components::BoidLabel;
use bevy::{gizmos, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::Velocity;

const BOID_MAX_SPEED: f32 = 200.0;
const BOID_MIN_SPEED: f32 = 20.0;

const BOID_MAX_Y: f32 = 200.0;

const FOLLOW_RADIUS: f32 = 200.0;
const AVOID_RADIUS: f32 = 40.0;
const AVOID_FACTOR: f32 = 0.05;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FACTOR: f32 = 0.0005;

#[derive(Debug, PartialEq, Default)]
struct BoidCompute {
    close: Vec2,
    velocity_sum: Vec2,
    position_sum: Vec2,
    neighbors: f32,
}

// boids: https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html
pub fn update_boid(
    mut gizmos: Gizmos,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut boid_query: Query<(Entity, &mut Transform, &mut Velocity), With<BoidLabel>>,
) {
    let window = window_query.single();
    let max_y = window.height() / 2.0;
    let min_y = -max_y;
    let max_x = window.width() / 2.0;
    let min_x = -max_x;

    let mut compute_table = HashMap::<Entity, RefCell<BoidCompute>>::default();

    // Apply the computation
    for [boid_1, boid_2] in boid_query.iter_combinations() {
        let diff = (boid_1.1.translation - boid_2.1.translation).truncate();
        let distance = diff.length();

        if distance < FOLLOW_RADIUS {
            let (compute1_cell, compute2_cell) =
                get_compute(&boid_1.0, &boid_2.0, &mut compute_table);

            let compute1 = &mut *compute1_cell.borrow_mut();
            let compute2 = &mut *compute2_cell.borrow_mut();

            if distance < AVOID_RADIUS {
                // Seperation
                compute1.close += diff;
                compute2.close -= diff;

                // gizmos.line(boid_1.1.translation, boid_2.1.translation, Color::RED)
            } else {
                // Alignment
                compute1.position_sum += boid_2.1.translation.truncate();
                compute2.position_sum += boid_1.1.translation.truncate();
                compute1.velocity_sum += boid_2.2.linvel;
                compute2.velocity_sum += boid_1.2.linvel;
                compute1.neighbors += 1.0;
                compute2.neighbors += 1.0;

                // gizmos.line(boid_1.1.translation, boid_2.1.translation, Color::GREEN)
            }
        } else {
            // gizmos.line(boid_1.1.translation, boid_2.1.translation, Color::WHITE)
        }
    }

    // Apply the computation
    for (entity, mut t, mut v) in boid_query.iter_mut() {
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

        let speed = v.linvel.length();

        if speed == 0.0 {
            v.linvel = Vec2::Y * BOID_MAX_SPEED;
        } else if speed < BOID_MIN_SPEED {
            v.linvel = v.linvel.normalize() * BOID_MIN_SPEED;
        } else if speed > BOID_MAX_SPEED {
            v.linvel = v.linvel.normalize() * BOID_MAX_SPEED;
        }

        // Update position
        // TODO: You need the average velocity of the frame, and delta_second is the velocity of the last frame, so this is wrong
        t.translation += v.linvel.extend(0.0) * time.delta_seconds();

        let new_rotation = Quat::from_rotation_z(Vec2::Y.angle_between(v.linvel));

        if new_rotation.angle_between(t.rotation) < 0.1 {
            v.linvel = (v.linvel * 1.1).clamp_length(BOID_MIN_SPEED, BOID_MAX_SPEED)
        }

        t.rotation = new_rotation;

        if t.translation.y > max_y {
            t.translation.y = min_y;
        } else if t.translation.y < min_y {
            t.translation.y = max_y;
        }

        if t.translation.x > max_x {
            t.translation.x = min_x;
        } else if t.translation.x < min_x {
            t.translation.x = max_x;
        }
    }
}

// This is wrong right? The the returned reference is only valid until insert is called on the HashMap...
// Or... The values live... Like you can't remove the entity from the HashMap without dropping the value...
fn get_compute<'a>(
    entity1: &Entity,
    entity2: &Entity,
    compute_table: &'a mut HashMap<Entity, RefCell<BoidCompute>>,
) -> (&'a RefCell<BoidCompute>, &'a RefCell<BoidCompute>) {
    if !compute_table.contains_key(entity1) {
        compute_table.insert(*entity1, RefCell::new(BoidCompute::default()));
    }

    if !compute_table.contains_key(entity2) {
        compute_table.insert(*entity2, RefCell::new(BoidCompute::default()));
    }
    return (
        compute_table.get(entity1).unwrap(),
        compute_table.get(entity2).unwrap(),
    );
}
