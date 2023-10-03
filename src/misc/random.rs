use std::f32::consts::PI;

use bevy::prelude::Vec2;
use rand::distributions::*;
use rand::prelude::*;

/// Generates a random point within a circle of the given radius, using the provided random number generator.
/// The point is returned as a 2D vector.
///
/// # Arguments
///
/// * `rng` - A mutable reference to a random number generator implementing the `Rng` trait.
/// * `radius` - The radius of the circle to generate the point within.
///
/// # Example
///
/// ```
/// use rand::prelude::*;
/// use crate::Vec2;
///
/// let mut rng = rand::thread_rng();
/// let point = uniform_circle(&mut rng, 5.0);
/// assert!(point.magnitude() <= 5.0);
/// ```
pub fn uniform_circle<R>(rng: &mut R, radius: f32) -> Vec2
where
    R: Rng + ?Sized,
{
    let uniform: Uniform<f32> = Uniform::new(0.0, 1.0);
    let r = radius * uniform.sample(rng).sqrt();
    let theta = uniform.sample(rng) * 2.0 * PI;

    let x = r * theta.cos();
    let y = r * theta.sin();
    Vec2::new(x, y)
}

/// Generates a random point within a donut shape defined by an outer radius and an inner radius, using the provided random number generator.
/// The point is returned as a 2D vector.
///
/// # Arguments
///
/// * `rng` - A mutable reference to a random number generator implementing the `Rng` trait.
/// * `out_radius` - The outer radius of the donut shape to generate the point within.
/// * `inner_radius` - The inner radius of the donut shape to generate the point within.
///
/// # Example
///
/// ```
/// use rand::prelude::*;
/// use crate::Vec2;
///
/// let mut rng = rand::thread_rng();
/// let point = uniform_donut(&mut rng, 5.0, 2.0);
/// assert!(point.magnitude() <= 5.0 && point.magnitude() >= 2.0);
/// ```
pub fn uniform_donut<R>(rng: &mut R, out_radius: f32, inner_radius: f32) -> Vec2
where
    R: Rng + ?Sized,
{
    loop {
        let candidate = uniform_circle(rng, out_radius);
        if candidate.length() > inner_radius {
            return candidate;
        }
    }
}
