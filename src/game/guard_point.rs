use bevy::prelude::*;

/// GuardPoint is a component that is used to mark a point on the map that a guard should guard.
///
/// For example, we add it to the kamikaze drone so they will try to stay in the center
#[derive(Debug, Clone, Copy, PartialEq, Component, Default)]
pub struct GuardPoint {
    pub point: Vec2,       // The point to guard
    pub max_distance: f32, // The maximum distance the guard can be from the point
}

impl GuardPoint {
    pub fn new(point: Vec2, max_distance: f32) -> Self {
        Self {
            point,
            max_distance,
        }
    }
}
