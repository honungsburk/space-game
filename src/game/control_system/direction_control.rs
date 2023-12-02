use bevy::prelude::*;
use bevy_rapier2d::{dynamics::ReadMassProperties, prelude::ExternalImpulse};

use crate::misc::control::PID;

/// Used to control an entity's rotation.
///
/// Warning: There must be an ExternalImpulse, ReadMassProperties, and Transform component on the entity.
#[derive(Component)]
pub struct DirectionControl {
    is_on: bool,
    control: PID,
    max_angular_acceleration: f32,
}

impl Default for DirectionControl {
    fn default() -> Self {
        Self {
            is_on: true,
            control: PID::rotation(0.005, 0.0, 0.005, 0.0),
            max_angular_acceleration: f32::MAX,
        }
    }
}

impl DirectionControl {
    pub fn new(control: PID, max_angular_acceleration: f32) -> Self {
        Self {
            control,
            max_angular_acceleration,
            ..default()
        }
    }

    /// Create a DirectionControl that will control the entity's rotation.
    ///
    /// # Arguments
    /// - `max_angular_acceleration`: The maximum angular acceleration of the entity. Measured in radians per second squared.
    ///
    /// # Returns
    /// A DirectionControl that will control the entity's rotation.
    ///
    /// # Links
    /// - [Angular Acceleration](https://en.wikipedia.org/wiki/Angular_acceleration)
    pub fn with_max_angular_acceleration(max_angular_acceleration: f32) -> Self {
        Self {
            max_angular_acceleration: max_angular_acceleration,
            ..default()
        }
    }

    pub fn calculate_torque_impule(
        &mut self,
        measured_value: f32,
        angular_inertia: f32,
        dt: f32,
    ) -> Option<f32> {
        if dt > 0.0 && self.is_on {
            let max = self.max_angular_acceleration * angular_inertia;
            Some(self.control.update(measured_value, dt).clamp(-max, max))
        } else {
            None
        }
    }

    pub fn set_setpoint(&mut self, setpoint: f32) {
        self.control.set_setpoint(setpoint);
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn is_off(&self) -> bool {
        !self.is_on
    }
}

pub fn update_direction_control(
    time: Res<Time>,
    mut query: Query<(
        &mut ExternalImpulse,
        &Transform,
        &mut DirectionControl,
        &ReadMassProperties,
    )>,
) {
    for (mut impulse, transform, mut direction_control, mass) in query.iter_mut() {
        let (_, _, current_angle) = transform.rotation.to_euler(EulerRot::XYZ);

        if let Some(torque_impulse) = direction_control.calculate_torque_impule(
            current_angle,
            mass.principal_inertia,
            time.delta_seconds(),
        ) {
            impulse.torque_impulse = torque_impulse;
        }
    }
}
