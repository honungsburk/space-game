use std::f32::consts::PI;

use bevy::prelude::Vec2;

// A PID controller is a control loop feedback mechanism widely used in industrial
// control systems and a variety of other applications requiring continuously modulated control.
//
// A PID controller continuously calculates an error value e(t) as the difference between a
// desired setpoint (SP) and a measured process variable (PV) and applies a correction based on proportional, integral,
// and derivative terms (denoted P, I, and D respectively) which give their name to the controller type.
#[derive(Debug)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    setpoint: f32,
    integral: f32,
    last_error: f32,
    compute_error: fn(f32, f32) -> f32,
}

impl PID {
    pub fn new(
        compute_error: fn(f32, f32) -> f32,
        kp: f32,
        ki: f32,
        kd: f32,
        setpoint: f32,
    ) -> Self {
        Self {
            compute_error: compute_error,
            kp,
            ki,
            kd,
            setpoint,
            integral: 0.0,
            last_error: 0.0,
        }
    }

    pub fn rotation(kp: f32, ki: f32, kd: f32, setpoint: f32) -> Self {
        Self::new(
            |setpoint, measured_value| {
                let normalized_setpoint: f32 = setpoint % (2.0 * PI);
                let normalized_measured_value: f32 = measured_value % (2.0 * PI);

                let diff = normalized_setpoint - normalized_measured_value;

                if diff > PI {
                    diff - 2.0 * PI
                } else if diff < -PI {
                    diff + 2.0 * PI
                } else {
                    diff
                }
            },
            kp,
            ki,
            kd,
            setpoint,
        )
    }

    pub fn basic(kp: f32, ki: f32, kd: f32, setpoint: f32) -> Self {
        Self::new(
            |setpoint, measured_value| setpoint - measured_value,
            kp,
            ki,
            kd,
            setpoint,
        )
    }

    pub fn update(&mut self, measured_value: f32, dt: f32) -> f32 {
        let error = (self.compute_error)(self.setpoint, measured_value);
        self.integral += error * dt;
        let derivative = (error - self.last_error) / dt;
        self.last_error = error;
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }

    pub fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }

    pub fn get_setpoint(&self) -> f32 {
        self.setpoint
    }

    pub fn add_to_setpoint(&mut self, amount: f32) {
        self.setpoint += amount;
    }
}

// 2D PID controller
#[derive(Debug)]
pub struct PID2D {
    pub x: PID,
    pub y: PID,
}

impl PID2D {
    pub fn new(x: PID, y: PID) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, measured_value: Vec2, dt: f32) -> Vec2 {
        Vec2::new(
            self.x.update(measured_value.x, dt),
            self.y.update(measured_value.y, dt),
        )
    }

    pub fn update_xy(&mut self, x: f32, y: f32, dt: f32) -> (f32, f32) {
        (self.x.update(x, dt), self.y.update(y, dt))
    }

    pub fn set_setpoint(&mut self, setpoint: Vec2) {
        self.x.set_setpoint(setpoint.x);
        self.y.set_setpoint(setpoint.y);
    }

    pub fn set_setpoint_xy(&mut self, x: f32, y: f32) {
        self.x.set_setpoint(x);
        self.y.set_setpoint(y);
    }

    pub fn get_setpoint(&self) -> Vec2 {
        Vec2::new(self.x.get_setpoint(), self.y.get_setpoint())
    }

    pub fn get_setpoint_xy(&self) -> (f32, f32) {
        (self.x.get_setpoint(), self.y.get_setpoint())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_rotation_pid_controller() {
        let mut pid = PID::rotation(1.0, 0.0, 0.0, 0.0);
        assert_eq!(pid.get_setpoint(), 0.0);

        let output = pid.update(0.5, 0.1);
        assert_eq!(output, -0.5);

        pid.set_setpoint(1.5);
        assert_eq!(pid.get_setpoint(), 1.5);

        let output = pid.update(1.0, 0.1);
        assert_eq!(output, 0.5);

        pid.set_setpoint(1.1 + 2.0 * PI * 5.0);

        let output = pid.update(1.0, 0.1);
        relative_eq!(output, 0.1);

        pid.set_setpoint(1.1 - 2.0 * PI * 5.0);

        let output = pid.update(1.0, 0.1);
        relative_eq!(output, 0.1);
    }
}
