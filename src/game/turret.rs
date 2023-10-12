use crate::{
    misc::control::{PID, PID2D},
    parent_child_no_rotation::{NoRotationChild, NoRotationParent},
};

use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

use super::{
    assets::{self, AssetDB},
    components::Health,
    player::components::Player,
};
use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    geometry::*,
    prelude::{ExternalForce, ExternalImpulse, RigidBody, Velocity},
};
pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_turret).add_systems(
            Update,
            (
                update_turret_target,
                update_turret_rotation.before(update_stationary_control),
                update_stationary_control,
                update_turret_radius_outline.after(update_turret_target),
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
pub struct Target {
    pub sees_target: bool,
    pub target: Vec2,
}

impl Target {
    pub fn new(sees_target: bool, target: Vec2) -> Self {
        Self {
            sees_target,
            target,
        }
    }
}

impl Default for Target {
    fn default() -> Self {
        Self {
            sees_target: false,
            target: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
struct TurretRadiusOutline {}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

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
        &mut RotationControl,
        &Transform,
        &Target,
        &mut ExternalImpulse,
    )>,
    time: Res<Time>,
) {
    for (mut rotation_control, turret_transform, target, mut turret_impulse) in query.iter_mut() {
        if target.target == Vec2::ZERO {
            continue;
        }

        let dt = time.delta_seconds();
        let desired_angel =
            Vec2::Y.angle_between(target.target - turret_transform.translation.xy());

        rotation_control.control.set_setpoint(desired_angel);

        let (_, _, current_angle) = turret_transform.rotation.to_euler(EulerRot::XYZ);

        let control_signal = rotation_control.control.update(current_angle, dt);

        turret_impulse.torque_impulse = control_signal * 0.001;
    }
}

// TODO: Replace this with a sensor from bevy_rapier2d
fn update_turret_target(
    mut query: Query<(&mut Target, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut target, transform) in query.iter_mut() {
            if transform.translation.distance(player_transform.translation) < 300.0 {
                target.target = player_transform.translation.xy();
                target.sees_target = true;
            } else {
                target.sees_target = false;
            }
        }
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

// Use events instead???
fn update_turret_radius_outline(
    turret_query: Query<&Target, With<Turret>>,
    mut turret_radius_query: Query<(&Parent, &mut Stroke), With<TurretRadiusOutline>>,
) {
    for (parent, mut stroke) in turret_radius_query.iter_mut() {
        if let Ok(target) = turret_query.get(parent.get()) {
            if target.sees_target {
                stroke.color = Color::rgba(1.0, 0.0, 0.0, 0.4);
            } else {
                stroke.color = Color::rgba(0.0, 0.0, 0.0, 0.2);
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
        // Properties
        .insert(Health::new(1))
        // Physics
        .insert(SpatialBundle::from_transform(spawn_transform))
        .insert(NoRotationParent)
        .insert(RigidBody::Dynamic)
        .insert(CollisionGroups::new(
            assets::ENEMY_GROUP.into(),
            assets::ENEMY_FILTER_MASK.into(),
        ))
        .insert(turret_base.collider.clone())
        .insert(SolverGroups::new(
            assets::ENEMY_GROUP.into(),
            assets::ENEMY_FILTER_MASK.into(),
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
        .insert(Target::default())
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
