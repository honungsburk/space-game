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

use std::collections::HashMap;
use std::f32::consts::PI;

use super::assets::groups;
use super::assets::AssetDB;
use super::config::Flag;
use super::control_system::DirectionControl;
use super::game_entity::Enemy;
use super::game_entity::GameEntityType;
use super::player::components::Player;
use super::vitality::Health;
use super::weapon::Weapon;
use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use bevy_rapier2d::prelude::*;
use rand_distr;
use rand_distr::Distribution;
use rand_distr::Poisson;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisionConeDebugFlag>()
            .add_systems(Update, (update_enemy_shooting, update_enemy));
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

#[derive(Resource, DerefMut, Deref)]
pub struct VisionConeDebugFlag {
    pub flag: Flag,
}

impl Default for VisionConeDebugFlag {
    fn default() -> Self {
        Self {
            flag: Flag::new("Vision Cone Debug", "Display Vision Cones", true),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

trait CastVisionCones {
    /// Casts a vision cone from the start position, and returns a list of entities
    /// that are in the vision cone.
    fn cast_vision_cone(
        &self,
        start: &Transform,           // start position
        ray_angel_density: f32, // number of rays to cast per radian (so put 180 for one ray per degree)
        inner_distance: f32,    // inner radius of the donut, so you do not collide with yourself
        outer_distance: f32,    // outer radius of the donut, where the vision ends
        angle: f32,             // angle of the vision cone, in radians
        gizmos: &mut Option<Gizmos>, // gizmos to draw the rays
    ) -> HashMap<Entity, Vec2>;
}

impl CastVisionCones for RapierContext {
    fn cast_vision_cone(
        &self,
        transform: &Transform,
        ray_angel_density: f32,
        inner_distance: f32,
        outer_distance: f32,
        angle: f32,
        gizmos: &mut Option<Gizmos>,
    ) -> HashMap<Entity, Vec2> {
        // Store entities, the sum of all the hit points, and the number of hits
        let mut entities: HashMap<Entity, (Vec2, u32)> = HashMap::new();

        let filter = QueryFilter::default(); // We must filter projectiles?
        let ray_max_toi = outer_distance - inner_distance;
        let mut ray_angle = -angle / 2.0;

        let start = transform.translation.truncate();
        let cone_direction = (transform.rotation * Vec3::Y).truncate();

        while ray_angle < angle / 2.0 {
            let direction = cone_direction.rotate(Vec2::new(ray_angle.cos(), ray_angle.sin()));

            let ray_start = start + direction * inner_distance;

            if let Some((entity, toi)) =
                self.cast_ray(ray_start, direction, ray_max_toi, true, filter)
            {
                let ray_end = ray_start + direction * toi;

                if let Some((sum, n)) = entities.get_mut(&entity) {
                    *sum += ray_end;
                    *n += 1;
                } else {
                    entities.insert(entity, (ray_end, 1));
                }

                if let Some(gizmos) = gizmos {
                    gizmos.line(ray_start.extend(0.0), ray_end.extend(0.0), Color::WHITE);
                }
            } else {
                if let Some(gizmos) = gizmos {
                    gizmos.line(
                        ray_start.extend(0.0),
                        (start + direction * outer_distance).extend(0.0),
                        Color::WHITE,
                    );
                }
            }

            ray_angle += 1.0 / ray_angel_density;
        }

        // Compute the average position of each entity

        let mut final_entities: HashMap<Entity, Vec2> = HashMap::new();

        for (entity, (sum, n)) in entities {
            final_entities.insert(entity, sum / n as f32);
        }

        final_entities
    }
}

fn update_enemy_shooting(
    time: Res<Time>,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut query: Query<(&mut ShootTimer, &Transform, &mut Weapon)>,
) {
    for (mut shoot_timer, transform, mut weapon) in query.iter_mut() {
        if shoot_timer.update(&time) {
            let position = transform.translation.truncate();
            let direction = (transform.rotation * Vec3::Y).truncate();

            weapon.fire(&mut commands, &asset_db, &asset_server, *transform);
        }
    }
}

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
        (With<EnemyShipLabel>, Without<Player>),
    >,
    player_query: Query<&Player>,
    rapier_context: Res<RapierContext>,
) {
    let mut giz = if vision_cone_debug.flag.is_on() {
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

        let visible_entities = rapier_context.cast_vision_cone(
            &enemy_transform,
            vision_donut_segment.ray_angel_density,
            vision_donut_segment.inner_distance,
            vision_donut_segment.outer_distance,
            angel, // TODO limit the angle based on the velocity of the enemy
            &mut giz,
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
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    spawn_location: Vec2,
    rotation: f32,
) -> Entity {
    let asset = &asset_db.enemy_ship_1;

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
        .insert(Velocity::zero())
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
        .insert(DirectionControl {
            torque_impulse_magnitude: 0.005,
            ..Default::default()
        })
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
        .insert(asset.collider.clone())
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
