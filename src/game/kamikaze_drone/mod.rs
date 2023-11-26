mod components;
mod systems;

use crate::misc::transform::from_location_angle;

use self::components::KamikazeDroneLabel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::{assets, guard_point::GuardPoint};

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
        Velocity::default(),
        KamikazeDroneLabel,
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
