mod components;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::misc::transform::from_location_angle;

use self::components::BoidLabel;

use super::assets::AssetDB;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_boid);
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

    let drone_entity = commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(Velocity::default())
        .insert(BoidLabel)
        .id();

    return drone_entity;
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<BoidLabel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
