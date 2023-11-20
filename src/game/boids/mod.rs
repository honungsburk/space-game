mod components;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use self::components::BoidLabel;
use crate::game::assets;
use crate::misc::transform::from_location_angle;

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
    asset_server: &Res<AssetServer>,
    location: Vec2,
    rotation: f32,
) -> Entity {
    let spawn_transform = from_location_angle(location, rotation);
    let asset = assets::KAMIKAZE_DRONE;

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
