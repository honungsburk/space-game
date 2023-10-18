mod ai;

use crate::{
    misc::control::{PID, PID2D},
    parent_child_no_rotation::{NoRotationChild, NoRotationParent},
};

use self::ai::TurretAI;

use super::{
    assets::{groups, AssetDB},
    game_entity::GameEntityType,
    player::components::Player,
    vitality::Health,
    weapon::Weapon,
};

use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{
    geometry::*,
    prelude::{CollisionEvent, ExternalForce, ExternalImpulse, RigidBody, Velocity},
};
use std::f32::consts::PI;
pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_turret).add_systems(
            Update,
            (
                update_turret_target,
                update_turret_rotation.before(update_stationary_control),
                register_turret_target,
                // update_stationary_control,
                update_turret_radius_outline,
                fire_weapon,
                update_ai.after(fire_weapon).after(update_turret_rotation),
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Turret;

// Used to control the player's rotation.
#[derive(Component)]
pub struct RotationControl {
    pub control: PID,
}

impl Default for RotationControl {
    fn default() -> Self {
        Self {
            control: PID::rotation(0.05, 0.0, 0.05, 0.0),
        }
    }
}

#[derive(Component)]
pub struct StationaryControl {
    pub control: PID2D,
}

impl Default for StationaryControl {
    fn default() -> Self {
        Self {
            control: PID2D::new(
                PID::basic(0.1, 0.0, 0.0, 0.0),
                PID::basic(0.1, 0.0, 0.0, 0.0),
            ),
        }
    }
}

#[derive(Component)]
pub struct Targets {
    targets: Vec<Target>,
}

#[derive(Debug, PartialEq)]
pub struct Target {
    entity: Entity,
    location: Vec2,
}

impl Targets {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    pub fn add(&mut self, target: Target) {
        if !self.targets.contains(&target) {
            self.targets.push(target);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        self.targets.retain(|e| e.entity != entity);
    }

    pub fn clear(&mut self) {
        self.targets.clear();
    }

    pub fn get_selected(&self) -> Option<&Target> {
        self.targets.first()
    }

    pub fn for_each(&mut self, f: impl Fn(&mut Target)) {
        self.targets.iter_mut().for_each(f);
    }
}

impl Default for Targets {
    fn default() -> Self {
        Targets::new()
    }
}

#[derive(Component)]
struct TurretRadiusOutline {}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_ai(mut query: Query<(&mut ai::TurretAI, &Targets)>, time: Res<Time>) {
    for (mut turret_ai, targets) in query.iter_mut() {
        turret_ai.state.update(&time, !targets.is_empty());
        println!("{:?}", turret_ai.state);
    }
}

fn fire_weapon(
    mut query: Query<(&ai::TurretAI, &mut Weapon, &Transform)>,
    mut commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
) {
    for (turret_ai, mut weapon, transform) in query.iter_mut() {
        if turret_ai.state.is_firing() {
            weapon.fire(&mut commands, &asset_db, &asset_server, *transform)
        }
    }
}

fn spawn_test_turret(
    mut commands: Commands,
    asset_db: Res<crate::game::assets::AssetDB>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let arena_center = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);

    let spawn_transform = Transform::from_translation(arena_center + Vec3::new(0.0, 300.0, 0.0));

    spawn_turret(&mut commands, &asset_db, &asset_server, spawn_transform);
}

fn update_turret_rotation(
    mut query: Query<(
        &TurretAI,
        &mut RotationControl,
        &Transform,
        &Targets,
        &mut ExternalImpulse,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    if dt == 0.0 {
        return;
    }
    for (turret_ai, mut rotation_control, turret_global_transform, targets, mut turret_impulse) in
        query.iter_mut()
    {
        if !turret_ai.state.is_targeting() {
            continue;
        }

        if let Some(target) = targets.get_selected() {
            let desired_angel =
                Vec2::Y.angle_between(target.location - turret_global_transform.translation.xy());

            // if target.location - turret_transform.translation().xy() == Vec2::ZERO then desired_angel is NaN
            if desired_angel.is_nan() {
                continue;
            }

            rotation_control.control.set_setpoint(desired_angel);

            let (_, _, current_angle) = turret_global_transform.rotation.to_euler(EulerRot::XYZ);

            let control_signal = rotation_control.control.update(current_angle, dt);

            turret_impulse.torque_impulse = control_signal * 0.001;
        }
    }
}

fn update_turret_target(
    mut target_query: Query<&mut Targets>,
    transform_query: Query<&GlobalTransform>,
) {
    for mut targets in target_query.iter_mut() {
        targets.for_each(|target| {
            if let Ok(target_transform) = transform_query.get(target.entity) {
                target.location = target_transform.translation().xy();
            }
        })
    }
}

// TODO: Create a custom event for this CollisionEvent => CustomEvent
// Then we only need to read through the events once, but will we be delayed one frame?
fn register_turret_target(
    mut collision_events: EventReader<CollisionEvent>,
    mut targets_query: Query<&mut Targets, Without<Player>>,
    sensor_query: Query<(&Parent, &Sensor)>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let sensor = sensor_query.get(*entity1).or(sensor_query.get(*entity2));

                if let Ok((parent, _)) = sensor {
                    let targets = targets_query.get_mut(parent.get());
                    let player_entity = if player_query.contains(*entity1) {
                        *entity1
                    } else {
                        *entity2
                    };

                    let player = player_query.get(player_entity);

                    if let (Ok(mut targets), Ok(player_global)) = (targets, player) {
                        let player_location = player_global.translation().xy();

                        let target = Target {
                            entity: player_entity,
                            location: player_location,
                        };

                        targets.add(target);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                let sensor = sensor_query.get(*entity1).or(sensor_query.get(*entity2));
                if let Ok((parent, _)) = sensor {
                    let targets = targets_query.get_mut(parent.get());
                    if let Ok(mut targets) = targets {
                        if player_query.contains(*entity1) {
                            targets.remove(*entity1);
                        } else if player_query.contains(*entity2) {
                            targets.remove(*entity2);
                        }
                    }
                }
            }
        }
    }
}

fn get_target<'a>(
    targets_query: &'a mut Query<&mut Targets, Without<Player>>,
    entity1: &Entity,
    entity2: &Entity,
) -> Option<Mut<'a, Targets>> {
    if targets_query.contains(*entity1) {
        return targets_query.get_mut(*entity1).ok();
    } else if targets_query.contains(*entity2) {
        return targets_query.get_mut(*entity2).ok();
    } else {
        return None;
    }
}

fn update_stationary_control(
    mut query: Query<(&mut StationaryControl, &Velocity, &mut ExternalImpulse)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    if dt == 0.0 {
        return;
    }

    for (mut stationary_control, turret_velocity, mut turret_impulse) in query.iter_mut() {
        if turret_velocity.linvel.length() == 0.0 {
            continue;
        }

        let control_signal = stationary_control
            .control
            .update(turret_velocity.linvel, dt);

        let new_impulse = (control_signal * 1.0).clamp_length_max(0.4);
        if new_impulse.length() > 0.0 {
            turret_impulse.impulse = new_impulse
            //TODO: add max impulse
        }
    }
}

fn update_turret_radius_outline(
    turret_query: Query<&Targets, With<Turret>>,
    mut turret_radius_query: Query<(&Parent, &mut Stroke), With<TurretRadiusOutline>>,
) {
    for (parent, mut stroke) in turret_radius_query.iter_mut() {
        if let Ok(targets) = turret_query.get(parent.get()) {
            if targets.is_empty() {
                stroke.color = Color::rgba(0.0, 0.0, 0.0, 0.2);
            } else {
                stroke.color = Color::rgba(1.0, 0.0, 0.0, 0.4);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Spawn
////////////////////////////////////////////////////////////////////////////////

fn spawn_turret(
    commands: &mut Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
) {
    let turret_base = &asset_db.turret_base_big;
    let gun = &asset_db.gun_8;

    commands
        .spawn(Turret)
        .insert(TurretAI::default())
        .insert(GameEntityType::Enemy)
        // Properties
        .insert(Health::at_max(1))
        // Physics
        .insert(SpatialBundle::from_transform(spawn_transform))
        .insert(NoRotationParent)
        .insert(RigidBody::Dynamic)
        .insert(CollisionGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(turret_base.collider.clone())
        .insert(SolverGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(Velocity { ..default() })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(RotationControl::default())
        .insert(StationaryControl::default())
        .insert(Targets::default())
        .insert(Weapon::simple_laser(
            groups::ENEMY_PROJECTILE_GROUP,
            groups::ENEMY_PROJECTILE_FILTER_MASK,
        ))
        .with_children(|parent| {
            let mut gun_transform = Transform::from_translation(Vec3::new(0.0, 20.0, 0.0));

            gun_transform.rotate(Quat::from_rotation_z(PI));

            parent.spawn(SpriteBundle {
                texture: asset_server.load(gun.sprite_path),
                transform: gun_transform,
                ..Default::default()
            });

            parent.spawn(SpriteBundle {
                texture: asset_server.load(turret_base.sprite_path),
                ..Default::default()
            });

            let grey = Color::rgba(0.0, 0.0, 0.0, 0.2);

            let mut stroke = Stroke::new(grey, 4.0);

            stroke.options.start_cap = LineCap::Round;
            stroke.options.end_cap = LineCap::Round;

            parent
                .spawn((dashed_circle(300.0, 10.0, 10.0), stroke))
                .insert(NoRotationChild)
                .insert(Collider::ball(300.0))
                .insert(ColliderMassProperties::Density(0.0))
                .insert(Sensor)
                .insert(CollisionGroups::new(
                    groups::ENEMY_GROUP.into(),
                    groups::PLAYER_GROUP.into(),
                ))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TurretRadiusOutline {});
        });
}

fn dashed_circle(radius: f32, dash_length: f32, gap_length: f32) -> ShapeBundle {
    // Build a Path.

    let mut path_builder = PathBuilder::new();
    let (dash_radians, gap_radians) = calculate_dash_gap_radians(radius, dash_length, gap_length);

    let mut total_radians = 0.0;

    while (total_radians + dash_radians) < (2.0 * PI) {
        path_builder.move_to(rotate_vec2(Vec2::new(0., radius), total_radians));
        path_builder.arc(
            Vec2::ZERO,
            Vec2::new(radius, radius),
            dash_radians,
            total_radians,
        );
        total_radians += dash_radians + gap_radians;
    }

    let path = path_builder.build();

    ShapeBundle { path, ..default() }
}

fn calculate_dash_gap_radians(radius: f32, dash_length: f32, gap_length: f32) -> (f32, f32) {
    let circumference = 2.0 * std::f32::consts::PI * radius;
    let dash_radians = (dash_length / circumference) * 2.0 * std::f32::consts::PI;
    let gap_radians = (gap_length / circumference) * 2.0 * std::f32::consts::PI;
    (dash_radians, gap_radians)
}

fn rotate_vec2(vec: Vec2, radians: f32) -> Vec2 {
    let cos_theta = radians.cos();
    let sin_theta = radians.sin();
    Vec2::new(
        vec.x * cos_theta - vec.y * sin_theta,
        vec.x * sin_theta + vec.y * cos_theta,
    )
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_rotate_vec2() {
        let vec = Vec2::new(1.0, 0.0);

        // Test rotating by 90 degrees
        let rotated_vec1 = rotate_vec2(vec, std::f32::consts::FRAC_PI_2);
        assert_relative_eq!(rotated_vec1.x, 0.0);
        assert_relative_eq!(rotated_vec1.y, 1.0);

        // Test rotating by 180 degrees
        let rotated_vec2 = rotate_vec2(vec, std::f32::consts::PI);
        assert_relative_eq!(rotated_vec2.x, -1.0);
        assert_relative_eq!(rotated_vec2.y, 0.0);

        // Test rotating by 270 degrees
        let rotated_vec3 = rotate_vec2(vec, 3.0 * std::f32::consts::FRAC_PI_2);
        assert_relative_eq!(rotated_vec3.x, 0.0);
        assert_relative_eq!(rotated_vec3.y, -1.0);
    }
}
