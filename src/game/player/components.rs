use std::f32::consts::PI;

use bevy::prelude::*;

use crate::misc::control::PID;

#[derive(Component)]
pub struct Player {}

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
                0.0,
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
