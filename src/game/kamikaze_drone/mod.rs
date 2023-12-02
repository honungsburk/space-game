mod components;
mod systems;

use crate::misc::{control::PID, transform::from_location_angle};

use self::components::KamikazeDroneLabel;
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{ExternalForce, ExternalImpulse, ReadMassProperties, RigidBody},
    prelude::Velocity,
};

use super::{
    assets,
    game_entity::GameEntityType,
    guard_point::GuardPoint,
    thrustor::{AngularThrustor, LinearThrustor},
    vitality::Health,
};

pub use components::KamikazeDroneTargetLabel;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct KamikazeDronesPlugin;

impl Plugin for KamikazeDronesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update);
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
    guard_point_opt: Option<GuardPoint>,
) -> Entity {
    let spawn_transform = from_location_angle(location, rotation);
    let asset = assets::KAMIKAZE_DRONE;

    let mut drone_entity = commands.spawn((
        SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        },
        // Labels
        KamikazeDroneLabel,
        GameEntityType::Enemy,
        // Stats
        Health::at_max(10),
        // Physics
        asset.collider(),
        ReadMassProperties::default(),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Velocity::default(),
        RigidBody::Dynamic,
        // Thrustors
        LinearThrustor::with_max_acceleration(1.0),
        AngularThrustor {
            max_angular_acceleration: 1.0,
            control: PID::rotation(0.00001, 0.0, 0.00001, 0.0),
            ..default()
        },
    ));

    if let Some(guard_point) = guard_point_opt {
        drone_entity.insert(guard_point);
    }

    return drone_entity.id();
}

/// Despawn all kamikaze drones
pub fn despawn(mut commands: Commands, query: Query<Entity, With<KamikazeDroneLabel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
