use std::f32::consts::PI;

use crate::misc::control::PID;

use super::{
    assets::{self, AssetDB},
    components::Health,
    player::components::Player,
};
use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    geometry::*,
    prelude::{ExternalForce, ExternalImpulse, RigidBody},
};
pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_turret)
            .add_systems(Update, (update_turret_target, update_turret_rotation));
        // Systems
        // On Exit State
        // .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
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

impl RotationControl {
    pub fn new(control: PID) -> Self {
        Self { control }
    }

    pub fn default() -> Self {
        Self {
            control: PID::rotation(0.5, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Component)]
pub struct Target {
    pub target: Vec2,
}

impl Target {
    pub fn new(target: Vec2) -> Self {
        Self { target }
    }
}

impl Default for Target {
    fn default() -> Self {
        Self { target: Vec2::ZERO }
    }
}

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

        turret_impulse.torque_impulse = control_signal * 0.005;
    }
}

fn update_turret_target(
    mut query: Query<&mut Target>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut target in query.iter_mut() {
            target.target = player_transform.translation.xy();
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
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(RotationControl::default())
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
        });

    // .add_child(turret_base_entity)
    // .add_child(turret_gun_entity);
}
