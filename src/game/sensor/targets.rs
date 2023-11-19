use bevy::prelude::{Component, Entity};
use hashlink::LinkedHashSet;

/// Contains all targets that a sensor has detected.
///
/// This is a component that should be added to a sensor entity.
#[derive(Debug, PartialEq, Eq, Component)]
pub struct SensorTargets {
    targets: LinkedHashSet<Entity>,
    has_changed: bool,
}

impl Default for SensorTargets {
    fn default() -> Self {
        Self::new()
    }
}

impl SensorTargets {
    /// Create a new SensorTargets.
    pub fn new() -> Self {
        Self {
            targets: LinkedHashSet::new(),
            has_changed: false,
        }
    }

    /// Checks if the state of the target has changed since the last time this method was called.
    /// If the state has changed, returns true. Otherwise, returns false.
    pub fn has_changed(&mut self) -> bool {
        let state = self.has_changed;
        self.has_changed = false;
        state
    }

    /// Create a new SensorTargets with a capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            targets: LinkedHashSet::with_capacity(capacity),
            has_changed: false,
        }
    }

    /// Check if the SensorTargets contains a target.
    pub fn contains(&self, target: &Entity) -> bool {
        self.targets.contains(target)
    }

    /// Add a target to the SensorTargets.
    pub fn insert(&mut self, target: Entity) -> bool {
        self.has_changed = true;
        self.targets.insert(target)
    }

    /// Remove a target from the SensorTargets.
    pub fn remove(&mut self, target: Entity) -> bool {
        self.has_changed = true;
        self.targets.remove(&target)
    }

    /// Clear all targets from the SensorTargets.
    pub fn clear(&mut self) {
        self.has_changed = true;
        self.targets.clear();
    }

    /// Iterate over the targets from the SensorTargets.
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.targets.iter()
    }

    /// Get the first target from the SensorTargets.
    pub fn front(&self) -> Option<&Entity> {
        self.targets.front()
    }

    /// Get the last target from the SensorTargets.
    pub fn back(&self) -> Option<&Entity> {
        self.targets.back()
    }

    /// Get last added target from the SensorTargets.
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Get the number of targets in the SensorTargets.
    pub fn len(&self) -> usize {
        self.targets.len()
    }
}
