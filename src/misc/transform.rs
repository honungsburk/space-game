use bevy::prelude::{Quat, Transform, Vec2};

/// A 2D transform
///
/// This is a helper function to create a 2D transform from a location and angle
///
/// # Arguments
///
/// * `location` - The location of the transform
/// * `angle` - The angle of the transform
///
/// # Returns
///
/// * `Transform` - The 2D transform
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
///
/// use space_game::misc::transform::from_location_angle;
///
/// let transform = from_location_angle(Vec2::new(0.0, 0.0), 0.0);
///
/// assert_eq!(transform.translation, Vec3::new(0.0, 0.0, 0.0));
/// assert_eq!(transform.rotation, Quat::from_rotation_z(0.0));
/// ```
pub fn from_location_angle(location: Vec2, angle: f32) -> Transform {
    Transform::from_translation(location.extend(0.0))
        .mul_transform(Transform::from_rotation(Quat::from_rotation_z(angle)))
}
