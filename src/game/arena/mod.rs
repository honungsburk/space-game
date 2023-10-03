use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::misc::random;
use crate::misc::sdf;
use crate::misc::sdf::SDF2D;
use bevy_rapier2d::prelude::*;

use super::assets::AssetDB;
use rand::distributions::*;
use rand::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_random_arena);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Arena;

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_random_arena(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // asset_db: Res<AssetDB>,
    // asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

    // First we need to generate a random arena size
    // let arena_size = 1000.0;

    // Then we need to generate a random arena shape
    // let arena_shape = sdf::circle(arena_size);

    // Walk the surface of the SDF and spawn asteroids

    commands
        .spawn(RigidBody::Fixed)
        .insert(TransformBundle::from(Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        )))
        .insert(hollow_circle(1000.0, 200))
        .insert(Arena);
}

fn hollow_circle(radius: f32, number_of_points: u32) -> Collider {
    // Generate
    let mut vertices: Vec<Vect> = Vec::new();

    for i in 0..number_of_points {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / number_of_points as f32;
        vertices.push(Vect::new(angle.cos() * radius, angle.sin() * radius));
    }
    // Close the loop
    vertices.push(vertices[0]);

    // let mut indices: Vec<usize> = Vec::new();

    // for i in 0..number_of_points {
    //     indices.push(i as usize);
    // }

    Collider::polyline(vertices, None)
}
