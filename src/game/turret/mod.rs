mod ai;
mod components;
mod draw;
mod systems;

use self::ai::TurretAI;
use super::game_entity::Enemy;
use super::{
    assets::{self, groups},
    game_entity::GameEntityType,
    sensor::ColliderSensorBundle,
    vitality::Health,
    weapon::Weapon,
};
use crate::{
    parent_child_no_rotation::{NoRotationChild, NoRotationParent},
    prelude::*,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{
    geometry::*,
    prelude::{ExternalForce, ExternalImpulse, RigidBody, Velocity},
};
use components::*;
use std::f32::consts::PI;

////////////////////////////////////////////////////////////////////////////////
/// Config
////////////////////////////////////////////////////////////////////////////////

pub struct TurretConfig {
    pub max_health: u32,
    pub weapon_damage: u32,
}

impl Default for TurretConfig {
    fn default() -> Self {
        Self {
            max_health: 30,
            weapon_damage: 10,
        }
    }
}

impl TurretConfig {
    pub fn new(max_health: u32, weapon_damage: u32) -> Self {
        Self {
            max_health,
            weapon_damage,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////
pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::update_turret_rotation.before(systems::update_stationary_control),
                // update_stationary_control,
                systems::update_turret_radius_outline,
                systems::fire_weapon,
                systems::update_ai
                    .after(systems::fire_weapon)
                    .after(systems::update_turret_rotation),
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Spawn & Despawn
////////////////////////////////////////////////////////////////////////////////

pub fn despawn(mut commands: Commands, query: Query<Entity, With<TurretLabel>>) {
    commands.despawn_all(&query);
}

pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    turret_config: &TurretConfig,
    spawn_transform: Transform,
) {
    let turret_base = assets::TURRET_BASE_BIG;
    let gun = assets::GUN_8;

    commands
        .spawn(TurretLabel)
        .insert(Enemy)
        .insert(TurretAI::default())
        .insert(GameEntityType::Enemy)
        // Properties
        .insert(Health::at_max(turret_config.max_health))
        // Physics
        .insert(SpatialBundle::from_transform(spawn_transform))
        .insert(NoRotationParent)
        .insert(RigidBody::Dynamic)
        .insert(CollisionGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(turret_base.collider())
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
        .insert(Weapon::laser(
            turret_config.weapon_damage,
            1000.0,
            Timer::from_seconds(1.0, TimerMode::Once),
            None,
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

            let sensor_range = 500.0;

            parent
                .spawn(ColliderSensorBundle::<Vec2>::ball(
                    sensor_range,
                    groups::PLAYER_GROUP,
                ))
                .insert((
                    draw::dashed_circle(sensor_range, 10.0, 10.0),
                    stroke,
                    NoRotationChild,
                ))
                .insert(TurretSensorLabel);
        });
}
