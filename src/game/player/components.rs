use std::time::Duration;

use bevy::prelude::*;

/// After the player is hit, they are invulnerable to damage and screenshake from contact forces for a short period of time.
/// The timer is reset whenever there is a contact. This is to allow the player to push objects out of the way without taking damage.
#[derive(Component)]
pub struct ContactForceInvulnerability(Timer);

impl ContactForceInvulnerability {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }

    pub fn is_invulnerable(&self) -> bool {
        !self.0.finished()
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Component)]
pub struct PlayerLabel;
