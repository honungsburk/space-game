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
    linvel: Vec2,
    angvel: f32,
    dt: f32,
    n_frames: f32, // The number of seconds before being the average velocity is the same as the current velocity (if it doesn't change)
}

impl AverageVelocity {
    pub fn new(n: u32) -> Self {
        Self {
            linvel: Vec2::ZERO,
            angvel: 0.0,
            dt: 0.000001, // Avoid divide by zero
            n_frames: n as f32,
        }
    }

    pub fn update(&mut self, linvel: Vec2, angvel: f32, dt: f32) {
        self.linvel += (linvel - self.linvel) * dt / self.n_frames;
        self.angvel += (angvel - self.angvel) * dt / self.n_frames;
        self.dt += (self.dt - dt) / self.n_frames;
    }

    pub fn get_linvel(&self) -> Vec2 {
        return self.linvel / self.dt;
    }

    pub fn get_angvel(&self) -> f32 {
        return self.angvel / self.dt;
    }
}

fn update_average_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut AverageVelocity)>) {
    for (velocity, mut average_velocity) in query.iter_mut() {
        average_velocity.update(velocity.linvel, velocity.angvel, time.delta_seconds());
    }
}
