use std::f32::consts::PI;

// A PID controller is a control loop feedback mechanism widely used in industrial
// control systems and a variety of other applications requiring continuously modulated control.
//
// A PID controller continuously calculates an error value e(t) as the difference between a
// desired setpoint (SP) and a measured process variable (PV) and applies a correction based on proportional, integral,
// and derivative terms (denoted P, I, and D respectively) which give their name to the controller type.
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
                let diff = setpoint - measured_value;

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
