mod components;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, ExternalForce, ExternalImpulse, Sensor, Velocity,
};

use crate::misc::transform::from_location_angle;

use self::components::{KamikazeDroneLabel, KamikazeDroneSensorLabel};

use super::assets::AssetDB;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct KamikazeDronesPlugin;

impl Plugin for KamikazeDronesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_kamikaze_drone);
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
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    location: Vec2,
    rotation: f32,
) -> Entity {
    let spawn_transform = from_location_angle(location, rotation);
    let asset = &asset_db.kamikaze_drone;
    let sensor_range = 200.0;

    let drone_sensor = commands
        .spawn(KamikazeDroneSensorLabel)
        .insert(Collider::ball(sensor_range))
        .insert(ColliderMassProperties::Density(0.0))
        .insert(Sensor)
        .id();

    let drone_entity = commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(Velocity::default())
        .insert(ExternalImpulse::default())
        .insert(ExternalForce::default())
        .insert(KamikazeDroneLabel)
        .push_children(&[drone_sensor])
        .id();

    return drone_entity;
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<KamikazeDroneLabel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
