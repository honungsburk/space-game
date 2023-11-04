use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::misc::control::PID;

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
pub struct Player;

// Used to control the player's rotation.
#[derive(Component)]
pub struct DirectionControl {
    pub is_enabled: bool,
    pub control: PID,
}

impl DirectionControl {
    pub fn new(is_enabled: bool, control: PID) -> Self {
        Self {
            is_enabled,
            control,
        }
    }

    pub fn default() -> Self {
        Self {
            is_enabled: false,
            control: PID::new(
                |setpoint, measured_value| {
                    let diff = setpoint - measured_value;

                    if diff > PI {
                        diff - 2.0 * PI
                    } else if diff < -PI {
                        diff + 2.0 * PI
                    } else {
                        diff
                    }
                },
                1.0,
                0.0,
                1.0,
                0.0,
            ),
        }
    }

    pub fn update(&mut self, measured_value: f32, dt: f32) -> Option<f32> {
        if self.is_enabled {
            Some(self.control.update(measured_value, dt))
        } else {
            None
        }
    }

    pub fn turn_off(&mut self) {
        self.is_enabled = false;
    }

    pub fn turn_on(&mut self) {
        self.is_enabled = true;
    }

    pub fn flip(&mut self) {
        self.is_enabled = !self.is_enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_setpoint(&mut self, setpoint: f32) {
        self.control.set_setpoint(setpoint);
    }
}
