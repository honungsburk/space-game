use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

use crate::game::player::PlayerLabel;

use super::{PickupEvent, PickupLabel, PickupRadius};

const PICKUP_IMPULSE_MAGNITUDE: f32 = 1.0;

pub fn attract_pickup(
    mut pickup_query: Query<(&Transform, &mut ExternalImpulse), With<PickupLabel>>,
    player_query: Query<(&Transform, &PickupRadius), Without<PickupLabel>>,
) {
    if let Ok((player_transform, player_pickup_radius)) = player_query.get_single() {
        for (pickup_transform, mut pickup_impulse) in pickup_query.iter_mut() {
            let diff = player_transform.translation - pickup_transform.translation;
            let distance = diff.length();

            if player_pickup_radius.can_pickup(distance) {
                pickup_impulse.impulse = diff.truncate().normalize() * PICKUP_IMPULSE_MAGNITUDE;
            }
        }
    }
}

pub fn pickup(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut pickup_events: EventWriter<PickupEvent>,
    projectile_query: Query<&PickupLabel>,
    player_query: Query<&PlayerLabel>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            // Will be removed before collision is resolved
            CollisionEvent::Started(entity1, entity2, flags) => {
                if flags.contains(CollisionEventFlags::REMOVED) {
                    continue;
                }

                let did_resolve = resolve_pickup(
                    &mut commands,
                    &mut pickup_events,
                    &projectile_query,
                    &player_query,
                    entity1,
                    entity2,
                );

                if !did_resolve {
                    resolve_pickup(
                        &mut commands,
                        &mut pickup_events,
                        &projectile_query,
                        &player_query,
                        entity2,
                        entity1,
                    );
                }
            }
            _ => {}
        }
    }
}

fn resolve_pickup(
    commands: &mut Commands,
    pickup_events: &mut EventWriter<PickupEvent>,
    pickup_query: &Query<&PickupLabel>,
    player_query: &Query<&PlayerLabel>,
    entity1: &Entity,
    entity2: &Entity,
) -> bool {
    if let Ok(pickup) = pickup_query.get(*entity1) {
        if let Ok(_) = player_query.get(*entity2) {
            commands.entity(*entity1).despawn_recursive();
            pickup_events.send(PickupEvent(pickup.kind.clone()));
            return true;
        }
        return false;
    }
    return false;
}
