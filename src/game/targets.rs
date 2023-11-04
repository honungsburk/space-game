//!
//! A component that lets an entity keep a list of targets that it can attack.
//!
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Targets {
    has_changed: bool,
    targets: Vec<Target>,
}

#[derive(Debug, PartialEq)]
pub struct Target {
    pub entity: Entity,
    pub location: Vec2,
}

impl Targets {
    /// Creates a new `Targets` instance.
    ///
    pub fn new() -> Self {
        Self {
            has_changed: true,
            targets: Vec::new(),
        }
    }

    /// Checks if the `targets` vector is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the `targets` vector is empty.
    /// * `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Adds a new `Target` to the `targets` vector if it does not already exist.
    ///
    /// # Arguments
    ///
    /// * `target` - A `Target` instance to be added to the `targets` vector.
    ///
    /// # Behavior
    ///
    /// If the `Target` instance is not already in the `targets` vector, it is added and `has_changed` is set to `true`.
    pub fn add(&mut self, target: Target) {
        if !self.targets.contains(&target) {
            self.targets.push(target);
            self.has_changed = true;
        }
    }

    /// Removes the target with the given entity from the list of targets.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to remove from the list of targets.
    ///
    pub fn remove(&mut self, entity: Entity) {
        self.targets.retain(|e| e.entity != entity);
        self.has_changed = true;
    }

    /// Clears the list of targets.
    pub fn clear(&mut self) {
        self.targets.clear();
        self.has_changed = true;
    }

    /// Returns the current target, if any.
    pub fn current_target(&self) -> Option<&Target> {
        self.targets.first()
    }

    /// Calls the given closure on each mutable reference to a `Target` in the collection.
    pub fn for_each(&mut self, f: impl Fn(&mut Target)) {
        self.targets.iter_mut().for_each(f);
    }

    /// Checks if the state of the target has changed since the last time this method was called.
    /// If the state has changed, returns true. Otherwise, returns false.
    pub fn has_changed(&mut self) -> bool {
        let state = self.has_changed;
        self.has_changed = false;
        state
    }

    /// Returns the number of targets.
    pub fn len(&self) -> usize {
        self.targets.len()
    }
}

impl Default for Targets {
    fn default() -> Self {
        Targets::new()
    }
}
