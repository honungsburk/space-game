use super::components::BoidLabel;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::Velocity;

const BOID_MAX_SPEED: f32 = 200.0;
const BOID_MIN_SPEED: f32 = 20.0;

const BOID_MAX_Y: f32 = 200.0;

// // boids: https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html
pub fn update_boid(
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut boid_query: Query<(Entity, &mut Transform, &mut Velocity), With<BoidLabel>>,
) {
    let window = window_query.single();
    let max_y = window.height() / 2.0;
    let min_y = -max_y;
    let max_x = window.width() / 2.0;
    let min_x = -max_x;

    for (_, mut t, mut v) in boid_query.iter_mut() {
        v.linvel = Vec2::new(0.0, 1.0) * BOID_MAX_SPEED;

        t.translation += v.linvel.extend(0.0) * time.delta_seconds();

        // Teleport to other side of screen if we go off screen

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
