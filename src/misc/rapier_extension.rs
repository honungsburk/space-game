use bevy::prelude::{Color, Entity, Gizmos, Transform, Vec2, Vec3};
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext};
use std::collections::HashMap;

/// Cast a vision cone from the given entity. Returns a map of entities to the average position of
/// the hit points.
///
/// # Parameters
///
/// * `ctx`: The rapier context
/// * `gizmos`: The gizmos to draw the vision cone on
/// * `entity_transform`: The transform of the entity casting the vision cone
/// * `ray_angel_density`: The number of rays to cast per degree
/// * `inner_distance`: The distance from the entity to start casting rays
/// * `outer_distance`: The distance from the entity to stop casting rays
/// * `angle`: The angle of the vision cone in degrees
///
/// # Returns
///
/// A map of entities to the average position of all the positions at which the vision cone hit it.
///
pub fn cast_vision_cone(
    ctx: &RapierContext,
    gizmos: &mut Option<Gizmos>,
    entity_transform: &Transform,
    ray_angel_density: f32,
    inner_distance: f32,
    outer_distance: f32,
    angle: f32,
) -> HashMap<Entity, Vec2> {
    // Store entities, the sum of all the hit points, and the number of hits
    let mut entities: HashMap<Entity, (Vec2, u32)> = HashMap::new();

    let filter = QueryFilter::default(); // We must filter projectiles?
    let ray_max_toi = outer_distance - inner_distance;
    let mut ray_angle = -angle / 2.0;

    let start = entity_transform.translation.truncate();
    let cone_direction = (entity_transform.rotation * Vec3::Y).truncate();

    while ray_angle < angle / 2.0 {
        let direction: Vec2 = cone_direction.rotate(Vec2::new(ray_angle.cos(), ray_angle.sin()));

        let ray_start = start + direction * inner_distance;

        if let Some((entity, toi)) = ctx.cast_ray(ray_start, direction, ray_max_toi, true, filter) {
            let ray_end = ray_start + direction * toi;

            if let Some((sum, n)) = entities.get_mut(&entity) {
                *sum += ray_end;
                *n += 1;
            } else {
                entities.insert(entity, (ray_end, 1));
            }

            if let Some(gizmos) = gizmos {
                gizmos.line(ray_start.extend(0.0), ray_end.extend(0.0), Color::WHITE);
            }
        } else {
            if let Some(gizmos) = gizmos {
                gizmos.line(
                    ray_start.extend(0.0),
                    (start + direction * outer_distance).extend(0.0),
                    Color::WHITE,
                );
            }
        }

        ray_angle += 1.0 / ray_angel_density;
    }

    // Compute the average position of each entity

    let mut final_entities: HashMap<Entity, Vec2> = HashMap::new();

    for (entity, (sum, n)) in entities {
        final_entities.insert(entity, sum / n as f32);
    }

    final_entities
}

/// Performs a ray cast starting at the direction the entity is facing. Returns the best path the entity
/// can take to avoid the obstacles in its way.
///
/// It works by performing shape casting (that is why you give it a width) and returns the first path
/// it finds that is clear of obstacles. We use a shape casting to make sure the path is wide enough
/// for the entity to fit through. If there are multiple paths, the one closest to the entity's
/// current direction is returned.
///
/// If no path is found, None is returned.
///
/// # Parameters
///
/// * `ctx`: The rapier context
/// * `gizmos`: The gizmos to draw cast shapes on
/// * `entity_transform`: The transform of the entity casting the vision cone
/// * `ray_angel_density`: The number of rays to cast per degree
/// * `inner_distance`: The distance from the entity to start shape casting
/// * `outer_distance`: The distance from the entity to stop shape casting
/// * `width`: The width of the entity
/// * `filter`: The filter to use when shape casting. This allows you to ignore certain entities.
/// * `max_angle`: The maximum angle we will shape cast for (angels are given in radians)
///
/// # Returns
///
/// The best angle to turn so the entity can take to avoid the obstacles in
/// its way. None if no angle is found.
///
pub fn find_unobstructed_path(
    ctx: &RapierContext,
    gizmos: &mut Option<Gizmos>,
    entity_transform: &Transform,
    ray_angel_density: f32,
    inner_distance: f32,
    outer_distance: f32,
    width: f32,
    filter: QueryFilter,
    max_angle: Option<f32>,
) -> Option<Vec2> {
    let entity_pos = entity_transform.translation.truncate();
    let entity_direction = (entity_transform.rotation * Vec3::Y).truncate();

    let shape = Collider::cuboid(width / 2.0, 1.0);
    let max_toi = outer_distance - inner_distance;

    let mut check_angle = |angle: f32| -> Option<Vec2> {
        let direction_vec: Vec2 = entity_direction
            .rotate(Vec2::new(angle.cos(), angle.sin()))
            .normalize();
        let shape_rot = direction_vec.y.atan2(direction_vec.x);

        let start_pos = entity_pos + direction_vec * inner_distance;

        let res = ctx.cast_shape(start_pos, shape_rot, direction_vec, &shape, max_toi, filter);

        // This gizmo is useless...
        if let Some((_, toi)) = res {
            if let Some(gizmo) = gizmos {
                gizmo.line(
                    start_pos.extend(0.0),
                    (start_pos + direction_vec * toi.toi).extend(0.0),
                    Color::RED,
                );
            }
        } else {
            if let Some(gizmo) = gizmos {
                gizmo.line(
                    start_pos.extend(0.0),
                    (start_pos + direction_vec * max_toi).extend(0.0),
                    Color::WHITE,
                );
            }
            return Some(direction_vec);
        }
        return None;
    };

    let max_angle_offset = max_angle.unwrap_or(std::f32::consts::PI);
    let mut current_angle_offset = 0.0;

    // Start shape casting at the current angle and work our way outwards
    while current_angle_offset < max_angle_offset {
        if let res @ Some(_) = check_angle(current_angle_offset) {
            return res;
        }
        if let res @ Some(_) = check_angle(-1.0 * current_angle_offset) {
            return res;
        }
        current_angle_offset += 1.0 / ray_angel_density;
    }

    return None;
}
