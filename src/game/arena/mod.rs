use crate::misc::random;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::assets;
use super::assets::AssetDB;
use super::meteors;
use super::meteors::MeteorSize;
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

pub const ARENA_RADIUS: f32 = 1000.0;
pub const PLAYER_SPAWN_RADIUS: f32 = 100.0;

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
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

    // First we need to generate a random arena size
    // let arena_size = 1000.0;

    // Then we need to generate a random arena shape
    // let arena_shape = sdf::circle(arena_size);

    // Walk the surface of the SDF and spawn asteroids

    // Create arena entity

    commands
        .spawn(RigidBody::Fixed)
        .insert(TransformBundle::from(Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        )))
        .insert(CollisionGroups::new(
            assets::ARENA_GROUP.into(),
            assets::ARENA_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            assets::ARENA_GROUP.into(),
            assets::ARENA_FILTER_MASK.into(),
        ))
        .insert(hollow_circle(ARENA_RADIUS, 200))
        .insert(Arena);

    spawn_random_meteors(&mut commands, &asset_db, &asset_server, window_query);

    // Add rocks
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

    Collider::polyline(vertices, None)
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_random_meteors(
    commands: &mut Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let arena_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    let mut rng = rand::thread_rng();

    for _ in 1..=20 {
        let size = rng.gen_range(0..10);

        let (meteor_size, meteor_radius) = match size {
            1..=2 => (MeteorSize::Tiny, assets::TINY_METEOR_RADIUS),
            3..=5 => (MeteorSize::Small, assets::SMALL_METEOR_RADIUS),
            6..=8 => (MeteorSize::Medium, assets::MEDIUM_METEOR_RADIUS),
            _ => (MeteorSize::Big, assets::BIG_METEOR_RADIUS),
        };

        // Subtract the meteor radius from the arena radius to ensure that the meteor is spawned
        // within the arena

        let candidate = arena_center
            + random::uniform_donut(&mut rng, ARENA_RADIUS - meteor_radius, PLAYER_SPAWN_RADIUS);
        let transform = Transform::from_xyz(candidate.x, candidate.y, 0.0);
        let is_movable = match meteor_size {
            MeteorSize::Tiny => true,
            MeteorSize::Small => true,
            MeteorSize::Medium => rng.gen_bool(0.7),
            MeteorSize::Big => rng.gen_bool(0.5),
        };
        if is_movable {
            meteors::spawn_meteor(
                asset_db,
                asset_server,
                commands,
                meteor_size,
                transform,
                Vec2::ZERO,
                0.0,
            );
        } else {
            meteors::spawn_immovable_meteor(
                asset_db,
                asset_server,
                commands,
                meteor_size,
                transform,
                // Vec2::ZERO,
                // 0.0,
            );
        }
    }
}
