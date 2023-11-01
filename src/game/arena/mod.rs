use super::assets;
use super::assets::AssetDB;
use super::camera::ScreenBounds;
use super::game_entity::Enemy;
use super::meteors;
use super::meteors::MeteorSize;
use super::turret;
use crate::misc::random;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::time::Duration;
////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::from_seconds(10.0))
            .insert_resource(Arena::new(2000.0, 400.0))
            .add_systems(Startup, spawn_random_arena)
            .add_systems(
                Update,
                (
                    tick_enemy_spawn_timer,
                    update_spawn_enemy.after(tick_enemy_spawn_timer),
                ),
            );
    }
}
pub const PLAYER_SPAWN_RADIUS: f32 = 100.0;

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }
}

impl EnemySpawnTimer {
    pub fn from_seconds(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Once),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn update_spawn_enemy(
    screen_bounds: Res<ScreenBounds>,
    enemy_query: Query<&Enemy>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    arena: Res<Arena>,
    mut commands: Commands,
    asset_db: Res<crate::game::assets::AssetDB>,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>,
) {
    if enemy_spawn_timer.timer.finished() && enemy_query.iter().count() < 100 {
        // Reduce the duration of the timer by 10% each time it finishes
        let duration = enemy_spawn_timer.timer.duration();
        enemy_spawn_timer
            .timer
            .set_duration(duration.mul_f32(0.9).max(Duration::from_secs_f32(1.0)));
        enemy_spawn_timer.timer.reset();

        // Spawn a new enemy
        spawn_enemy(
            &screen_bounds,
            &arena,
            &mut commands,
            &asset_db,
            &asset_server,
            rapier_context,
        );
    }
}

fn spawn_random_arena(
    mut commands: Commands,
    arena: Res<Arena>,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
) {
    arena.spawn_asteroid_bounds(&mut commands, &asset_db, &asset_server);
    spawn_random_meteors(&arena, &mut commands, &asset_db, &asset_server, 100);
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
    pub position: Vec2,
    pub rotation: f32,
    pub protcted_radius: f32,
}

/// At the edges of the arena, we want to spawn immovable asteroids to confine
/// the player to the arena. This struct describes the bounds of the arena.
/// The radius is the radius of the arena, and the width is the width of the
/// boundary.
struct AsteroidArenaBounds {
    radius: f32,
    width: f32,
}

impl AsteroidArenaBounds {
    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}

#[derive(Resource)]
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

    fn asteroid_bounds(&self) -> &AsteroidArenaBounds {
        &self.asteroid_bounds
    }

    fn player_spawn_locations(&self) -> &PlayerSpawnLocation {
        &self.player_spawn_locations
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

fn spawn_random_meteors(
    arena: &Res<Arena>,
    commands: &mut Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    number_of_meteors: usize,
) {
    let arena_center = Vec2::new(0.0, 0.0);

    let mut rng = rand::thread_rng();

    for _ in 1..=number_of_meteors {
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
            + random::uniform_donut(
                &mut rng,
                arena.asteroid_bounds.radius() - meteor_radius,
                arena.player_spawn_locations.protcted_radius,
            );
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
            );
        }
    }
}

fn spawn_enemy(
    screen_bounds: &Res<ScreenBounds>,
    arena: &Res<Arena>,
    commands: &mut Commands,
    asset_db: &Res<crate::game::assets::AssetDB>,
    asset_server: &Res<AssetServer>,
    rapier_context: Res<RapierContext>,
) {
    let mut has_spawn_location = false;

    let turret_asset = &asset_db.turret_base_big;
    let mut candidate_spawn_location = Vec2::new(0.0, 0.0);
    let filter = QueryFilter::default();

    // If we can't find a spawn location after 100 attempts, give up. The arena is probably full.
    let mut attempts = 0;
    let max_attempts = 100;

    // Try to find a valid spawn location
    while !has_spawn_location && attempts < max_attempts {
        attempts += 1;
        // Generate a candidate spawn location
        let mut rng = rand::thread_rng();
        candidate_spawn_location = random::uniform_circle(&mut rng, arena.asteroid_bounds.radius);

        // Spawn the turret outside of the screen
        if screen_bounds.contains(candidate_spawn_location) {
            continue;
        }

        has_spawn_location = true;

        // Check if the candidate spawn location is valid
        rapier_context.intersections_with_shape(
            candidate_spawn_location,
            0.0,
            &turret_asset.collider,
            filter,
            |entity| {
                has_spawn_location = false;
                false // Return `false` to stop the query.
            },
        );
    }

    if attempts < max_attempts {
        turret::spawn_turret(
            commands,
            asset_db,
            asset_server,
            Transform::from_xyz(candidate_spawn_location.x, candidate_spawn_location.y, 0.0),
        );
    }
}
