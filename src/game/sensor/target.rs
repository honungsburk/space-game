use bevy::prelude::*;

use super::SensorTargets;

/// A component that contains the current target of an entity.
///
/// Used by enemies to keep track of who they are attacking. By having this as a
/// seperate component we can easily make an entity attack one of its of kind.
///
/// For example, we can have turrets attack other turrets, or any other entity
/// of our choosing.
///
/// Note that the Target component is added as a child of the entity that is
/// targeting. This is done so that the entity can follow a target, that is itself
/// following another target.
///
#[derive(Debug, Component)]
pub struct SensorTarget<D>
where
    D: Send + Sync + 'static,
{
    has_changed: bool,
    target: Option<(Entity, D)>,
}

impl<D> Default for SensorTarget<D>
where
    D: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            has_changed: false,
            target: None,
        }
    }
}

impl<D> SensorTarget<D>
where
    D: Send + Sync + 'static,
{
    /// Checks if the state of the target has changed since the last time this method was called.
    /// If the state has changed, returns true. Otherwise, returns false.
    pub fn has_changed(&mut self) -> bool {
        let state = self.has_changed;
        self.has_changed = false;
        state
    }

    pub fn is_entity(&self, other: Entity) -> bool {
        self.get()
            .map(|(entity, _)| *entity == other)
            .unwrap_or(false)
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    /// Get the current target.
    pub fn get(&self) -> Option<&(Entity, D)> {
        self.target.as_ref()
    }

    /// Set the current target.
    pub fn set(&mut self, target: Entity, data: D) {
        self.has_changed = true;
        self.target = Some((target, data));
    }

    /// Clear the current target.
    pub fn clear(&mut self) {
        self.has_changed = self.target.is_some();
        self.target = None;
    }
}

// Specialization for Vec2

pub type SensorTargetVec2 = SensorTarget<Vec2>;

pub fn update_vec2_target(
    mut targets_query: Query<(&SensorTargets, &mut SensorTargetVec2)>,
    target_query: Query<&Transform>,
) {
    for (sensor_targets, mut target) in targets_query.iter_mut() {
        if let Some(current_target) = sensor_targets.front() {
            if let Ok(transform) = target_query.get(*current_target) {
                target.set(*current_target, transform.translation.xy());
            }
        } else {
            target.clear();
        }
    }
}
