//! Thrustor is a component that can be used to control the thrust of an entity.
//! It is used by the control system to apply forces to the entity to achieve the
//! desired velocity.
//!

pub struct AcceleratorThrustor {
    acceleration_curve: AccelerationCurve,
    weight: f32,
    max_acceleration: f32, // must be positive
    max_speed: f32,        // must be positive
}

impl AcceleratorThrustor {
    fn new(
        acceleration_curve: AccelerationCurve,
        weight: f32,
        max_acceleration: f32,
        max_speed: f32,
    ) -> Option<Self> {
        if max_acceleration <= 0.0 || max_speed <= 0.0 || weight <= 0.0 {
            return None;
        }

        Some(Self {
            acceleration_curve,
            weight,
            max_acceleration,
            max_speed,
        })
    }

    /// Returns the thrust that should be applied to the entity.
    fn thrust(&self, current_speed: f32) -> f32 {
        let relative_speed = (current_speed / self.max_speed).clamp(0.0, 1.0);

        let acceleration = self.max_acceleration * self.acceleration_curve.evaluate(relative_speed);

        let thrust = acceleration * self.weight;

        thrust
    }
}

enum AccelerationCurve {
    Linear { k: f32, m: f32 },
    EaseInOutCubic,
}

impl AccelerationCurve {
    /// Acceleration curve of the form: y = k * x + m
    ///
    /// In other words, the acceleration is proportional to the speed.
    fn linear(k: f32, m: f32) -> Self {
        Self::Linear { k, m }
    }

    /// Acceleration curve of the form: y = m
    ///
    /// In other words, the acceleration is constant in regards to speed.
    fn constant(m: f32) -> Self {
        Self::Linear { k: 0.0, m }
    }

    fn ease_in_out_cubic() -> Self {
        Self::EaseInOutCubic
    }

    fn evaluate(&self, speed: f32) -> f32 {
        match self {
            Self::Linear { k, m } => k * speed + m,
            Self::EaseInOutCubic => {
                // https://easings.net/#easeInOutCubic
                let t = speed;
                if t < 0.5 {
                    1.0 - 4.0 * t * t * t
                } else {
                    let t = -2.0 * t + 2.0;
                    t * t * t / 2.0
                }
            }
        }
    }
}

struct RotationThrustor {
    max_thrust: f32,       // must be positive
    max_acceleration: f32, // must be positive
    max_speed: f32,        // must be positive
}
