use super::assets;
use super::assets::AssetDB;
use super::meteors;
use super::meteors::MeteorSize;
use crate::misc::random;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::PI;
////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_random_arena);
    }
}

pub const ARENA_RADIUS: f32 = 2000.0;
pub const ARENA_BORDER_WIDTH: f32 = 400.0;
pub const PLAYER_SPAWN_RADIUS: f32 = 100.0;

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

// #[derive(Component)]
// pub struct Arena;

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_random_arena(
    mut commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
) {
    let arena = Arena::new(ARENA_RADIUS, ARENA_BORDER_WIDTH);

    arena.spawn_asteroid_bounds(&mut commands, &asset_db, &asset_server);

    // let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

    // First we need to generate a random arena size
    // let arena_size = 1000.0;

    // Then we need to generate a random arena shape
    // let arena_shape = sdf::circle(arena_size);

    // Walk the surface of the SDF and spawn asteroids

    // Create arena entity

    // commands
    //     .spawn(RigidBody::Fixed)
    //     .insert(TransformBundle::from(Transform::from_xyz(
    //         window.width() / 2.0,
    //         window.height() / 2.0,
    //         0.0,
    //     )))
    //     .insert(CollisionGroups::new(
    //         groups::ARENA_GROUP.into(),
    //         groups::ARENA_FILTER_MASK.into(),
    //     ))
    //     .insert(SolverGroups::new(
    //         groups::ARENA_GROUP.into(),
    //         groups::ARENA_FILTER_MASK.into(),
    //     ))
    //     .insert(hollow_circle(ARENA_RADIUS, 200));
    // .insert(Arena);

    // spawn_random_meteors(&mut commands, &asset_db, &asset_server, window_query);

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
// Arena System
////////////////////////////////////////////////////////////////////////////////

struct PlayerSpawnLocation {
    position: Vec2,
    rotation: f32,
    protcted_radius: f32,
}

/// At the edges of the arena, we want to spawn immovable asteroids to confine
/// the player to the arena. This struct describes the bounds of the arena.
/// The radius is the radius of the arena, and the width is the width of the
/// boundary.
struct AsteroidArenaBounds {
    radius: f32,
    width: f32,
}

struct Arena {
    asteroid_bounds: AsteroidArenaBounds,
    player_spawn_locations: PlayerSpawnLocation,
}

impl Arena {
    fn new(radius: f32, width: f32) -> Self {
        Self {
            asteroid_bounds: AsteroidArenaBounds { radius, width },
            player_spawn_locations: PlayerSpawnLocation {
                position: Vec2::ZERO,
                rotation: 0.0,
                protcted_radius: 100.0,
            },
        }
    }

    fn spawn_asteroid_bounds(
        &self,
        commands: &mut Commands,
        asset_db: &Res<AssetDB>,
        asset_server: &Res<AssetServer>,
    ) {
        let asteroid_bounds = &self.asteroid_bounds;

        let inner_radius = asteroid_bounds.radius + assets::BIG_METEOR_RADIUS * 2.0;
        let outer_radius = inner_radius + asteroid_bounds.width;

        let mut candidates = VecDeque::from([(Vec2::new(0.0, inner_radius), MeteorSize::Big)]);
        let mut added = Vec::new();

        let uniform: Uniform<f32> = Uniform::new(0.0, 1.0);

        let mut rng = rand::thread_rng();

        while let Some((candidate_pos, candidate_size)) = candidates.pop_front() {
            if candidate_pos.length() > outer_radius || candidate_pos.length() < inner_radius {
                continue;
            }

            // random rotation
            let rotation = uniform.sample(&mut rng) * 2.0 * PI;

            // Check if the candidate is valid (a.k.a. not colliding with anything)
            let valid = added.iter().all(|added_pos| {
                let diff: Vec2 = *added_pos - candidate_pos;
                let distance: f32 = diff.length();
                distance > assets::BIG_METEOR_RADIUS * 2.0
            });

            // can be replaced with AABB testing to make it faster, but I don't think it is needed

            if valid {
                added.push(candidate_pos);

                let mut transform = Transform::from_xyz(candidate_pos.x, candidate_pos.y, 0.0);

                transform.rotation = Quat::from_rotation_z(rotation);
                // Add the candidate to the world
                meteors::spawn_immovable_meteor(
                    asset_db,
                    asset_server,
                    commands,
                    candidate_size,
                    transform,
                );
                // generate more candidates

                let number_of_candidates = 6;

                let angle_offset = uniform.sample(&mut rng) * 2.0 * PI;

                for i in 1..=number_of_candidates {
                    let angle = 2.0 * PI * (i as f32 / number_of_candidates as f32) + angle_offset;
                    // 2 is so that there is no overlapp, 0.1 to add a bit of padding.
                    let distance = assets::BIG_METEOR_RADIUS * 2.1
                        + uniform.sample(&mut rng) * assets::BIG_METEOR_RADIUS * 0.2;
                    let offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);

                    candidates.push_back((offset + candidate_pos, MeteorSize::Big));
                }
            }
        }
    }
}

fn circle_area(radius: f32) -> f32 {
    radius * radius * std::f32::consts::PI
}

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
