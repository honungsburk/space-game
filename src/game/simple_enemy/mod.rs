mod components;
mod systems;

use crate::misc::{control::PID, transform::from_location_angle};

use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{ExternalForce, ExternalImpulse, ReadMassProperties, RigidBody},
    geometry::{ActiveEvents, CollisionGroups},
    prelude::Velocity,
};

use super::{
    assets::{self, groups},
    game_entity::GameEntityType,
    item_drop::ItemDropBuilder,
    thrustor::{AngularThrustor, LinearThrustor},
    vitality::{Damage, Health},
};

pub use components::SimpleEnemyLabel;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct SimpleEnemyPlugin;

impl Plugin for SimpleEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (systems::update, systems::update_on_collision));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Spawn & Despawn
////////////////////////////////////////////////////////////////////////////////

///
/// Spawn a kamikaze drone at the given location and rotation
///
pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    location: Vec2,
    rotation: f32,
) -> Entity {
    let spawn_transform = from_location_angle(location, rotation);
    let asset = assets::KAMIKAZE_DRONE;
    let drop = ItemDropBuilder::new().add_experience(10).build();

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: asset_server.load(asset.sprite_path),
                ..Default::default()
            },
            // Labels
            SimpleEnemyLabel,
            GameEntityType::Enemy,
            // Stats
            Health::at_max(20),
            Damage::new(10),
            // Physics
            asset.collider(),
            ReadMassProperties::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
            Velocity::default(),
            RigidBody::Dynamic,
            // Thrustors
            LinearThrustor::with_max_acceleration(100.0),
            AngularThrustor {
                max_angular_acceleration: 5.0,
                control: PID::rotation(0.00001, 0.0, 0.00001, 0.0),
                ..default()
            },
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(
                groups::ENEMY_GROUP.into(),
                groups::PLAYER_GROUP
                    .union(groups::PLAYER_PROJECTILE_GROUP)
                    .into(),
            ),
        ))
        .insert(drop)
        .id()
}

/// Despawn all kamikaze drones
pub fn despawn(mut commands: Commands, query: Query<Entity, With<SimpleEnemyLabel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
