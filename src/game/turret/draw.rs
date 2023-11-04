use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{PathBuilder, ShapeBundle};

/// Draw a dashed circle with the given radius, dash length, and gap length.
///
pub fn dashed_circle(radius: f32, dash_length: f32, gap_length: f32) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    let (dash_radians, gap_radians) = calculate_dash_gap_radians(radius, dash_length, gap_length);

    let mut total_radians = 0.0;

    while (total_radians + dash_radians) < (2.0 * PI) {
        path_builder.move_to(rotate_vec2(Vec2::new(0., radius), total_radians));
        path_builder.arc(
            Vec2::ZERO,
            Vec2::new(radius, radius),
            dash_radians,
            total_radians,
        );
        total_radians += dash_radians + gap_radians;
    }

    let path = path_builder.build();

    ShapeBundle { path, ..default() }
}

fn calculate_dash_gap_radians(radius: f32, dash_length: f32, gap_length: f32) -> (f32, f32) {
    let circumference = 2.0 * std::f32::consts::PI * radius;
    let dash_radians = (dash_length / circumference) * 2.0 * std::f32::consts::PI;
    let gap_radians = (gap_length / circumference) * 2.0 * std::f32::consts::PI;
    (dash_radians, gap_radians)
}

fn rotate_vec2(vec: Vec2, radians: f32) -> Vec2 {
    let cos_theta = radians.cos();
    let sin_theta = radians.sin();
    Vec2::new(
        vec.x * cos_theta - vec.y * sin_theta,
        vec.x * sin_theta + vec.y * cos_theta,
    )
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_rotate_vec2() {
        let vec = Vec2::new(1.0, 0.0);

        // Test rotating by 90 degrees
        let rotated_vec1 = rotate_vec2(vec, std::f32::consts::FRAC_PI_2);
        assert_relative_eq!(rotated_vec1.x, 0.0);
        assert_relative_eq!(rotated_vec1.y, 1.0);

        // Test rotating by 180 degrees
        let rotated_vec2 = rotate_vec2(vec, std::f32::consts::PI);
        assert_relative_eq!(rotated_vec2.x, -1.0);
        assert_relative_eq!(rotated_vec2.y, 0.0);

        // Test rotating by 270 degrees
        let rotated_vec3 = rotate_vec2(vec, 3.0 * std::f32::consts::FRAC_PI_2);
        assert_relative_eq!(rotated_vec3.x, 0.0);
        assert_relative_eq!(rotated_vec3.y, -1.0);
    }
}
