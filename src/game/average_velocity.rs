use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
pub struct AverageVelocityPlugin;

impl Plugin for AverageVelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_average_velocity);
    }
}

// Average Velocity

/**
 * Keep track of the average velocity of an entity. This is useful for
 * detecting changes in velocity such as when a player hits another object
 * or when a player is moving.
 *
 * Example uses:
 * - Calcualte the magnitude of the screen shake on collisions.
 */
#[derive(Component)]
pub struct AverageVelocity {
    samples: Vec<VelocitySample>,
    // The amound of time to average over
    linvel: Vec2,
    angvel: f32,
    seconds: f32,
}

struct VelocitySample {
    linvel: Vec2,
    angvel: f32,
    time: f32,
}

impl AverageVelocity {
    pub fn new(seconds: f32) -> Self {
        Self {
            samples: Vec::new(),
            linvel: Vec2::ZERO,
            angvel: 0.0,
            seconds: seconds,
        }
    }

    pub fn update(&mut self, linvel: Vec2, angvel: f32, dt: f32) {
        let mut total_time = dt;
        let mut new_samples = vec![VelocitySample {
            linvel,
            angvel,
            time: dt,
        }];
        let mut new_linvel = linvel * (dt / self.seconds).min(1.0);
        let mut new_angvel = angvel * (dt / self.seconds).min(1.0);

        while total_time < self.seconds {
            let time_left = self.seconds - total_time;
            if let Some(sample) = self.samples.pop() {
                let dt = sample.time.min(time_left);
                let pct_of_interval = (dt / self.seconds).min(1.0);
                new_linvel += sample.linvel * pct_of_interval;
                new_angvel += sample.angvel * pct_of_interval;
                new_samples.push(VelocitySample {
                    linvel: sample.linvel,
                    angvel: sample.angvel,
                    time: dt,
                });
                total_time += dt;
            } else {
                break;
            }
        }

        self.samples = new_samples;
        self.linvel = new_linvel;
        self.angvel = new_angvel;
    }

    pub fn get_linvel(&self) -> Vec2 {
        return self.linvel;
    }

    pub fn get_angvel(&self) -> f32 {
        return self.angvel;
    }
}

fn update_average_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut AverageVelocity)>) {
    for (velocity, mut average_velocity) in query.iter_mut() {
        average_velocity.update(velocity.linvel, velocity.angvel, time.delta_seconds());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_velocity_update_smaller_then_window() {
        let mut average_velocity = AverageVelocity::new(1.0);
        let linvel = Vec2::new(1.0, 1.0);
        let angvel = 3.0;
        let dt: f32 = 0.1;

        average_velocity.update(linvel, angvel, dt);

        assert_eq!(average_velocity.get_linvel(), linvel * dt);
        assert_eq!(average_velocity.get_angvel(), angvel * dt);

        average_velocity.update(linvel, angvel, dt);
        average_velocity.update(linvel, angvel, dt);

        assert_eq!(average_velocity.get_linvel(), linvel * 3.0 * dt);
        assert_eq!(average_velocity.get_angvel(), angvel * 3.0 * dt);
    }

    #[test]
    fn test_average_velocity_update_for_larger_then_window() {
        let mut average_velocity = AverageVelocity::new(1.0);
        let linvel = Vec2::new(1.0, 1.0);
        let angvel = 3.0;
        let dt: f32 = 2.0;

        average_velocity.update(linvel, angvel, dt);

        assert_eq!(average_velocity.get_linvel(), linvel);
        assert_eq!(average_velocity.get_angvel(), angvel);
    }
}
