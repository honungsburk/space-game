use bevy::prelude::*;

/// Control Camera with WASD or arrow keys
///
/// - `speed`: Movement speed in pixels per second
///
#[derive(Component, Debug, Clone, PartialEq)]
struct KeyboardMovement {
    speed: f32,
}

impl Default for KeyboardMovement {
    fn default() -> Self {
        Self { speed: 100.0 }
    }
}

pub fn update(query: Query<(&mut Transform, &KeyboardMovement)>) {}
