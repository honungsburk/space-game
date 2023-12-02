//! Thrustor is a component that can be used to control the thrust of an entity.
//! It is used by the control system to apply forces to the entity to achieve the
//! desired velocity.
//!
use crate::misc::control::{PID, PID2D};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{ReadMassProperties, Velocity},
    prelude::ExternalImpulse,
};

/// Represents a linear thrustor component used for controlling linear acceleration in a game.
/// It allows you to specify a desired velocity, and it will apply forces to the entity to
/// achieve the desired velocity.
///
/// Warning: There must be an ExternalImpulse, ReadMassProperties, and Velocity component on the entity.
#[derive(Component, Debug)]
pub struct LinearThrustor {
    control: PID2D,
    max_acceleration: f32, // must be positive
}

impl Default for LinearThrustor {
    fn default() -> Self {
        let kp: f32 = 0.005;
        let ki = 0.0;
        let kd = 0.005;
        let setpoint = 0.0;

        Self {
            control: PID2D::new(
                PID::basic(kp, ki, kd, setpoint),
                PID::basic(kp, ki, kd, setpoint),
            ),
            max_acceleration: f32::MAX,
        }
    }
}

impl LinearThrustor {
    /// Creates a new `LinearThrustor` with the specified PID controllers for the x and y axes,
    /// and the maximum acceleration allowed.
    ///
    /// # Arguments
    ///
    /// * `x_pid` - The PID controller for the x axis.
    /// * `y_pid` - The PID controller for the y axis.
    /// * `max_acceleration` - The maximum acceleration allowed.
    ///
    /// # Panics
    ///
    /// Panics if `max_acceleration` is less than or equal to 0.0.
    ///
    /// # Returns
    ///
    /// A new `LinearThrustor` instance.
    pub fn new(x_pid: PID, y_pid: PID, max_acceleration: f32) -> Self {
        if max_acceleration <= 0.0 {
            panic!("max_acceleration must be positive");
        }

        Self {
            control: PID2D::new(x_pid, y_pid),
            max_acceleration,
        }
    }

    /// Creates a new `LinearThrustor` with the specified maximum acceleration.
    ///
    /// # Arguments
    ///
    /// * `max_acceleration` - The maximum acceleration of the `LinearThrustor`.
    ///
    /// # Returns
    ///
    /// A new `LinearThrustor` instance with the specified maximum acceleration.
    pub fn with_max_acceleration(max_acceleration: f32) -> Self {
        Self {
            max_acceleration,
            ..default()
        }
    }

    pub fn calculate_linear_impulse(&mut self, velocity: Vec2, mass: f32, dt: f32) -> Vec2 {
        if dt > 0.0 {
            let max = self.max_acceleration * mass;
            self.control.update(velocity, dt).clamp_length_max(max)
        } else {
            Vec2::ZERO
        }
    }

    /// Sets the desired velocity for the linear thrustor.
    ///
    /// # Arguments
    ///
    /// * `desired_velocity` - The desired velocity as a `Vec2` containing the x and y components.
    ///
    /// # Example
    ///
    /// ```
    /// use game::control_system::linear_thrustor::LinearThrustor;
    /// use game::math::Vec2;
    ///
    /// let mut thrustor = LinearThrustor::new();
    /// let desired_velocity = Vec2::new(10.0, 5.0);
    /// thrustor.set_desired_velocity(desired_velocity);
    /// ```
    pub fn set_desired_velocity(&mut self, desired_velocity: Vec2) {
        self.control.x.set_setpoint(desired_velocity.x);
        self.control.y.set_setpoint(desired_velocity.y);
    }
}

/// Updates the linear thrustor components.
pub fn update(
    time: Res<Time>,
    mut query: Query<(
        &mut ExternalImpulse,
        &Velocity,
        &mut LinearThrustor,
        &ReadMassProperties,
    )>,
) {
    for (mut impulse, velocity, mut linear_thrustor, mass) in query.iter_mut() {
        impulse.impulse = linear_thrustor.calculate_linear_impulse(
            velocity.linvel,
            mass.principal_inertia,
            time.delta_seconds(),
        );
    }
}
