use bevy::prelude::*;
use bevy_rapier2d::{dynamics::ReadMassProperties, prelude::ExternalImpulse};

use crate::misc::control::PID;

/// Used to control an entity's rotation.
///
/// It allows you to specify a desired angle, and it will apply forces to the entity to
/// achieve the desired angle.
///
/// Warning: There must be an ExternalImpulse, ReadMassProperties, and Transform component on the entity.
#[derive(Component, Debug)]
pub struct AngularThrustor {
    pub is_on: bool,
    pub control: PID,
    pub max_angular_acceleration: f32,
}

impl Default for AngularThrustor {
    fn default() -> Self {
        Self {
            is_on: true,
            control: PID::rotation(0.005, 0.0, 0.005, 0.0),
            max_angular_acceleration: f32::MAX,
        }
    }
}

impl AngularThrustor {
    /// Creates a new `AngularThrustor` with the specified control parameters and maximum angular acceleration.
    ///
    /// # Arguments
    ///
    /// * `control` - The PID controller used for controlling the angular thrust.
    /// * `max_angular_acceleration` - The maximum angular acceleration allowed for the thrustor.
    ///
    /// # Returns
    ///
    /// A new `AngularThrustor` instance.
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

    pub fn calculate_torque_impulse(
        &mut self,
        angle: f32,
        angular_inertia: f32,
        dt: f32,
    ) -> Option<f32> {
        if dt > 0.0 && self.is_on {
            let max = self.max_angular_acceleration * angular_inertia;
            Some(self.control.update(angle, dt).clamp(-max, max))
        } else {
            None
        }
    }

    /// Sets the desired angle for the angular thrustor.
    ///
    /// # Arguments
    ///
    /// * `desired_angle` - The desired angle in radians.
    pub fn set_desired_angle(&mut self, desired_angle: f32) {
        self.control.set_setpoint(desired_angle);
    }

    /// Turns on the angular thrustor.
    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    /// Turns off the angular thrustor.
    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    /// Returns whether the angular thrustor is turned on or off.
    pub fn is_on(&self) -> bool {
        self.is_on
    }

    /// Checks if the angular thrustor is turned off.
    pub fn is_off(&self) -> bool {
        !self.is_on
    }
}

/// A bevy system to update the angular thrustor.
pub fn update(
    time: Res<Time>,
    mut query: Query<(
        &mut ExternalImpulse,
        &Transform,
        &mut AngularThrustor,
        &ReadMassProperties,
    )>,
) {
    for (mut impulse, transform, mut direction_control, mass) in query.iter_mut() {
        let (_, _, current_angle) = transform.rotation.to_euler(EulerRot::XYZ);

        if let Some(torque_impulse) = direction_control.calculate_torque_impulse(
            current_angle,
            mass.principal_inertia,
            time.delta_seconds(),
        ) {
            impulse.torque_impulse = torque_impulse;
        }
    }
}
