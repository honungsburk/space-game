use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalImpulse;

use crate::misc::control::PID;

/// Used to control an entity's rotation.
///
/// Warning: There myust be an ExternalImpulse and Transform component on the entity.
#[derive(Component)]
pub struct DirectionControl {
    pub is_enabled: bool,
    pub control: PID,
    pub torque_impulse_magnitude: f32,
    pub torque_impulse_max: f32,
}

impl Default for DirectionControl {
    fn default() -> Self {
        Self {
            is_enabled: true,
            control: PID::rotation(1.0, 0.0, 1.0, 0.0),
            torque_impulse_magnitude: 1.0,
            torque_impulse_max: f32::MAX,
        }
    }
}

impl DirectionControl {
    pub fn new(
        is_enabled: bool,
        control: PID,
        torque_impulse_magnitude: f32,
        torque_impulse_max: f32,
    ) -> Self {
        Self {
            is_enabled,
            control,
            torque_impulse_magnitude,
            torque_impulse_max,
        }
    }

    pub fn calculate_torque_impule(&mut self, measured_value: f32, dt: f32) -> Option<f32> {
        if self.is_enabled && dt > 0.0 {
            Some(
                (self.control.update(measured_value, dt) * self.torque_impulse_magnitude)
                    .clamp(-self.torque_impulse_max, self.torque_impulse_max),
            )
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

pub fn update_direction_control(
    time: Res<Time>,
    mut query: Query<(&mut ExternalImpulse, &Transform, &mut DirectionControl)>,
) {
    for (mut player_impulse, player_transform, mut direction_control) in query.iter_mut() {
        let (_, _, current_angle) = player_transform.rotation.to_euler(EulerRot::XYZ);

        if let Some(torque_impulse) =
            direction_control.calculate_torque_impule(current_angle, time.delta_seconds())
        {
            player_impulse.torque_impulse = torque_impulse;
        }
    }
}
