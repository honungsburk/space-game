use crate::misc::control::{PID, PID2D};
use bevy::prelude::*;

/// Label to identify a turret.
#[derive(Component)]
pub struct TurretLabel;

// Controlls the turret's rotation.
#[derive(Component)]
pub struct RotationControl {
    pub control: PID,
}

impl Default for RotationControl {
    fn default() -> Self {
        Self {
            control: PID::rotation(0.05, 0.0, 0.05, 0.0),
        }
    }
}

/// Tries to keep the turret stationary.
#[derive(Component)]
pub struct StationaryControl {
    pub control: PID2D,
}

impl Default for StationaryControl {
    fn default() -> Self {
        Self {
            control: PID2D::new(
                PID::basic(0.1, 0.0, 0.0, 0.0),
                PID::basic(0.1, 0.0, 0.0, 0.0),
            ),
        }
    }
}

/// Label to identify a turret's sensor.
#[derive(Component)]
pub struct TurretSensorLabel;
