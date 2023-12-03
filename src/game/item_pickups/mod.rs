mod component;
mod event;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Damping;
use bevy_rapier2d::dynamics::ExternalImpulse;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::ActiveEvents;
use bevy_rapier2d::geometry::CollisionGroups;
pub use component::PickupLabel;
pub use component::PickupRadius;
pub use event::PickupEvent;

use super::assets::groups;

pub struct ItemPickupPlugin;

impl Plugin for ItemPickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickupEvent>()
            .add_systems(Update, (systems::attract_pickup, systems::pickup));
    }
}

/// The different kinds of pickups
#[derive(Debug, PartialEq, Hash, Component, Clone)]
pub enum Pickup {
    Experience(u64),
}

/// Spawns a pickup at the given position.
pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pickup: Pickup,
    position: Vec2,
    rotation: f32,
) {
    let asset = match pickup {
        Pickup::Experience(_) => &crate::game::assets::PICKUP_EXPERIENCE,
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.0)
                .with_rotation(Quat::from_rotation_z(rotation)),
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        },
        PickupLabel::new(pickup),
        // Collider
        asset.collider(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(groups::PICKUP_GROUP.into(), groups::PLAYER_GROUP.into()),
        // Physics
        RigidBody::Dynamic,
        Damping {
            linear_damping: 1.0,
            angular_damping: 0.5,
        },
        ExternalImpulse::default(),
    ));
}
