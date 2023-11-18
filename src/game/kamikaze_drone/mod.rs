mod components;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::Damping,
    prelude::{
        ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, ExternalForce,
        ExternalImpulse, RigidBody, Sensor, SolverGroups, Velocity,
    },
};

use crate::misc::transform::from_location_angle;

use self::components::{BoidTargets, KamikazeDroneLabel, KamikazeDroneSensorLabel};

use super::assets::{groups, AssetDB};

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct KamikazeDronesPlugin;

impl Plugin for KamikazeDronesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::update_kamikaze_drone,
                systems::update_kamikaze_drone_targets,
            ),
        );
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
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(
            groups::SENSOR_GROUP.into(),
            groups::KAMIKAZE_DRONE_GROUP.into(),
        ))
        .id();

    let drone_entity = commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(KamikazeDroneLabel)
        .insert(asset.collider.clone())
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 1.0, // TODO: This should be 0.0 but we do not have a any controler for angualr velocity yet!
        })
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::default())
        .insert(ExternalImpulse::default())
        .insert(ExternalForce::default())
        .insert(BoidTargets::default())
        .insert(CollisionGroups::new(
            groups::KAMIKAZE_DRONE_GROUP.into(),
            groups::KAMIKAZE_DRONE_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::KAMIKAZE_DRONE_GROUP.into(),
            groups::KAMIKAZE_DRONE_FILTER_MASK.into(),
        ))
        .push_children(&[drone_sensor])
        .id();

    return drone_entity;
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<KamikazeDroneLabel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
