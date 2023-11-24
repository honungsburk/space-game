use bevy::{
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3},
    render::color::Color,
};

/**
 * Draws a crosshair at the given position.
 *
 * # Parameters
 * - `gizmos`: The gizmos object to draw the crosshair on.
 * - `position`: The position of the crosshair.
 * - `color`: The color of the crosshair.
 * - `half_width`: The half-width of the crosshair.
 */
pub fn crosshair(gizmos: &mut Gizmos, position: &Vec2, color: Color, half_width: f32) {
    let x = position.x;
    let y = position.y;

    gizmos.line(
        Vec3::new(x - half_width, y, 0.0),
        Vec3::new(x + half_width, y, 0.0),
        color,
    );
    gizmos.line(
        Vec3::new(x, y - half_width, 0.0),
        Vec3::new(x, y + half_width, 0.0),
        color,
    );
}
