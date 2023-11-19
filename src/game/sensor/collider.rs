use bevy::prelude::*;

use super::SensorTarget;
use super::SensorTargets;
use crate::game::assets::groups;
use bevy_rapier2d::{geometry::*, prelude::CollisionEvent};

////////////////////////////////////////////////////////////////////////////////
// Label
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Default)]
pub struct ColliderSensorLabel;

////////////////////////////////////////////////////////////////////////////////
// Circular Sensor Bundle
////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct ColliderSensorBundle<D>
where
    D: Send + Sync + 'static,
{
    pub circle: Collider,
    pub active_events: ActiveEvents,
    pub mass: ColliderMassProperties,
    pub sensor: Sensor,
    pub collision_groups: CollisionGroups,
    pub label: ColliderSensorLabel,
    pub targets: SensorTargets,
    pub target: SensorTarget<D>,
}

impl<D> Default for ColliderSensorBundle<D>
where
    D: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            circle: Collider::ball(500.0),
            active_events: ActiveEvents::COLLISION_EVENTS,
            mass: ColliderMassProperties::Density(0.0),
            collision_groups: CollisionGroups::default(),
            sensor: Sensor,
            label: ColliderSensorLabel,
            targets: SensorTargets::default(),
            target: SensorTarget::default(),
        }
    }
}

impl<D> ColliderSensorBundle<D>
where
    D: Send + Sync + 'static,
{
    pub fn ball(radius: f32, filters: Group) -> Self {
        Self {
            circle: Collider::ball(radius),
            collision_groups: CollisionGroups::new(groups::SENSOR_GROUP.into(), filters.into()),
            ..Default::default()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Update
////////////////////////////////////////////////////////////////////////////////

pub fn update_targets(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut SensorTargets, With<ColliderSensorLabel>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if let Ok(mut targets) = query.get_mut(*entity1) {
                    targets.insert(*entity2);
                }

                if let Ok(mut targets) = query.get_mut(*entity2) {
                    targets.insert(*entity1);
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if let Ok(mut targets) = query.get_mut(*entity1) {
                    targets.remove(*entity2);
                }

                if let Ok(mut targets) = query.get_mut(*entity2) {
                    targets.remove(*entity1);
                }
            }
        }
    }
}
