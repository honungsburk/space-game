pub mod components;
mod systems;

use crate::game::average_velocity::AverageVelocity;
use crate::game::control_system::DirectionControl;
use crate::game::game_entity::GameEntityType;
use crate::game::trauma::Trauma;
use crate::game::vitality::Health;
use crate::game::{assets::groups, assets::AssetDB, weapon::Weapon};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use systems::*;

pub use components::Player;

use self::components::ContactForceInvulnerability;

use super::camera::CameraTargetLabel;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                control_ship,
                fire_weapon,
                player_collision,
                update_contact_force_invulnerability,
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Spawning
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player_at_center(
    commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
) {
    spawn(Vec2::new(0.0, 0.0), std::f32::consts::PI / 2.0)(commands, asset_db, asset_server);
}

pub fn spawn(location: Vec2, rotation: f32) -> impl Fn(Commands, Res<AssetDB>, Res<AssetServer>) {
    move |mut commands, asset_db, asset_server| {
        spawn_player(&mut commands, &asset_db, &asset_server, location, rotation)
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    location: Vec2,
    rotation: f32,
) {
    // Spawn transform
    let spawn_transform = Transform::from_xyz(location.x, location.y, 0.0)
        .with_rotation(Quat::from_rotation_z(rotation));

    // Add the player entity

    commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(asset_db.player_ship.sprite_path),
            ..default()
        })
        .insert(Player {})
        .insert(GameEntityType::Player)
        .insert(DirectionControl {
            torque_impulse_magnitude: 0.005,
            ..Default::default()
        })
        .insert(CameraTargetLabel)
        .insert(RigidBody::Dynamic)
        .insert(asset_db.player_ship.collider.clone())
        .insert(Trauma::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceInvulnerability::new(0.1))
        .insert(ReadMassProperties::default())
        .insert(ContactForceEventThreshold(0.0)) // TODO: increase this to some reasonable value
        .insert(Health::at_max(100))
        .insert(CollisionGroups::new(
            groups::PLAYER_GROUP.into(),
            groups::PLAYER_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::PLAYER_GROUP.into(),
            groups::PLAYER_FILTER_MASK.into(),
        ))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        })
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(AverageVelocity::new(0.5))
        .insert(Weapon::laser(
            10,
            1000.0,
            Timer::from_seconds(1.0, TimerMode::Once),
            Timer::from_seconds(0.1, TimerMode::Repeating),
            groups::PLAYER_PROJECTILE_GROUP,
            groups::PLAYER_PROJECTILE_FILTER_MASK,
        ));
}

pub fn despawn(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    despawn_all(&mut commands, &player_query)
}

pub fn despawn_all(commands: &mut Commands, player_query: &Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}
