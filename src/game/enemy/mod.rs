//! # Enemy Ship
//!
//! ## Behavior
//!
//! The movement works with attraction and repulsion forces. The enemy ship is
//! attracted to the player ship and repulsed non-player entities. The enemy sees in a cone
//! in front of it. If the player is in the cone, the enemy will move towards the player.
//!
//! **Rules:**
//! - Moves forward
//! - If there are more then one enemy entity, move towards the first one that was seen (the influence is constant).
//! - If an entity is in the view cone (includes enemies), move away from it (the influence: -1 / distance^2).
//! - If there is an enemy entity in the view cone, shoot.
//!
//!

mod ai;

use super::assets;
use super::assets::groups;
use super::control_system::DirectionControl;
use super::debug::VisionConeDebugFlag;
use super::game_entity::Enemy;
use super::game_entity::GameEntityType;
use super::player::components::PlayerLabel;
use super::vitality::Health;
use super::weapon::Weapon;
use crate::misc::rapier_extension;
use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use bevy_rapier2d::prelude::*;
use rand_distr;
use rand_distr::Distribution;
use rand_distr::Poisson;
use std::f32::consts::PI;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_enemy);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component, Debug)]
struct ShootTimer {
    timer: Timer,
}

const POISSON_LAMBDA: f32 = 1.0;

impl Default for ShootTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl ShootTimer {
    pub fn new() -> Self {
        let seconds = Poisson::new(POISSON_LAMBDA)
            .unwrap()
            .sample(&mut rand::thread_rng());

        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }

    pub fn update(&mut self, time: &Time) -> bool {
        let completed = self.timer.tick(time.delta()).just_finished();

        if completed {
            let seconds = Poisson::new(POISSON_LAMBDA)
                .unwrap()
                .sample(&mut rand::thread_rng());
            self.timer = Timer::from_seconds(seconds, TimerMode::Once);
        }

        return completed;
    }
}

#[derive(Component, Debug)]
struct ShipNavigationSystem {
    /// If present the ship will be incentivized to be close to the guard point
    ///
    /// The first element is the point, and the second is the influence. A higher
    /// influence means the ship will be more incentivized to be close to the point,
    /// and a lower influence means the ship will be less incentivized to be close to it.
    ///
    guard_point: Option<(Vec2, f32)>,

    /// For each frame we calculate a rolling average of the influence vectors
    /// This is to smooth out the influence vector, so the ship does not jitter
    /// to much.
    average_influence: Vec2,
}

impl Default for ShipNavigationSystem {
    fn default() -> Self {
        Self {
            guard_point: None,
            average_influence: Vec2::ZERO,
        }
    }
}

impl ShipNavigationSystem {
    /// Returns the influence vector for the ship
    fn influence_vector(&self) -> Vec2 {
        self.average_influence
    }

    /// Sets the guard point for the ship
    fn set_guard_point(&mut self, point: Vec2, influence: f32) {
        self.guard_point = Some((point, influence));
    }

    /// Clears the guard point for the ship
    fn clear_guard_point(&mut self) {
        self.guard_point = None;
    }

    /// Update and return the influence vector for the ship
    fn update(&mut self, transform: &Transform, influence: &Vec2) -> Vec2 {
        let position = transform.translation.truncate();
        let forward_bias = (transform.rotation * Vec3::Y).truncate();
        self.average_influence = (self.average_influence + *influence + forward_bias) / 2.0;

        if let Some((guard_point, influence)) = self.guard_point {
            return position.distance(guard_point) * influence + self.average_influence;
        } else {
            return self.average_influence;
        }
    }
}

#[derive(Component, Debug)]
struct Tracking {
    entities: Vec<Entity>,
}

#[derive(Component, Debug)]
struct EnemyShipLabel;

#[derive(Component, Debug)]
struct VisionDonutSegment {
    ray_angel_density: f32,
    inner_distance: f32,
    outer_distance: f32,
    angle: f32,
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

// fn update_enemy_shooting(
//     time: Res<Time>,
//     asset_db: Res<AssetDB>,
//     asset_server: Res<AssetServer>,
//     mut commands: Commands,
//     mut query: Query<(&mut ShootTimer, &Transform, &mut Weapon)>,
// ) {
//     for (mut shoot_timer, transform, mut weapon) in query.iter_mut() {
//         if shoot_timer.update(&time) {
//             let position = transform.translation.truncate();
//             let direction = (transform.rotation * Vec3::Y).truncate();

//             weapon.fire(&mut commands, &asset_db, &asset_server, *transform);
//         }
//     }
// }

fn update_enemy(
    vision_cone_debug: Res<VisionConeDebugFlag>,
    gizmos: Gizmos, // TODO: expensive to pass this around?
    mut ship_query: Query<
        (
            &mut ExternalImpulse,
            &Velocity,
            &Transform,
            &VisionDonutSegment,
            &mut DirectionControl,
            &mut ShipNavigationSystem,
        ),
        (With<EnemyShipLabel>, Without<PlayerLabel>),
    >,
    player_query: Query<&PlayerLabel>,
    rapier_context: Res<RapierContext>,
) {
    let mut giz = if vision_cone_debug.is_on() {
        Some(gizmos)
    } else {
        None
    };

    for (
        mut enemy_impulse,
        velocity,
        enemy_transform,
        vision_donut_segment,
        mut direction_control,
        mut ship_navigation_system,
    ) in ship_query.iter_mut()
    {
        // Bias the influence vector towards the direction the enemy is facing
        // let (_, _, current_angle) = enemy_transform.rotation.to_euler(EulerRot::XYZ);
        let mut influence_vector = Vec2::ZERO;

        // Find every entity in the vision cone
        let angel = vision_donut_segment.angle * (velocity.linvel.length() / 200.0).clamp(0.0, 1.0);

        let visible_entities = rapier_extension::cast_vision_cone(
            &rapier_context,
            &mut giz,
            &enemy_transform,
            vision_donut_segment.ray_angel_density,
            vision_donut_segment.inner_distance,
            vision_donut_segment.outer_distance,
            angel,
        );

        for (visible_entity, visible_position) in visible_entities {
            // If the entity is the player, move towards it
            let ship_position = enemy_transform.translation.truncate();
            let distance = visible_position.distance(ship_position);
            let direction = (ship_position - visible_position).normalize();
            let influence = 100.0 / distance;

            let vel = direction * influence;

            if player_query.contains(visible_entity) {
                influence_vector -= vel;
            } else {
                influence_vector += vel;
            }
        }

        let final_influence = ship_navigation_system.update(&enemy_transform, &influence_vector);

        // turn the influence vector into an angle
        let new_angle = Vec2::Y.angle_between(final_influence);
        direction_control.set_setpoint(new_angle);

        enemy_impulse.impulse = final_influence;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Spawn
////////////////////////////////////////////////////////////////////////////////

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    spawn_location: Vec2,
    rotation: f32,
) -> Entity {
    let asset = assets::ENEMY_SHIP_1;

    let mut spawn_transform = Transform::from_translation(spawn_location.extend(0.0));
    spawn_transform.rotate_local_z(rotation);

    let entity = commands
        .spawn(SpriteBundle {
            texture: asset_server.load(asset.sprite_path),
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                // And don't flip it upside-down ( the default )
                flip_y: true,
                ..default()
            },
            transform: spawn_transform,
            ..Default::default()
        })
        .insert(ShootTimer::default())
        .insert(GameEntityType::Enemy)
        .insert(Enemy)
        .insert(EnemyShipLabel)
        .insert(VisionDonutSegment {
            ray_angel_density: 20.0,
            inner_distance: 60.0,
            outer_distance: 300.0,
            angle: PI / 2.0,
        })
        .insert(ShipNavigationSystem::default())
        .insert(DirectionControl::with_max_angular_acceleration(1.0))
        .insert(ReadMassProperties::default())
        .insert(Health::at_max(50))
        .insert(Weapon::laser(
            10,
            750.0,
            Timer::from_seconds(1.0, TimerMode::Once),
            None,
            groups::ENEMY_PROJECTILE_GROUP,
            groups::ENEMY_PROJECTILE_FILTER_MASK,
        ))
        .insert(RigidBody::Dynamic)
        .insert(asset.collider())
        .insert(CollisionGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .id();

    return entity;
}
