use bevy::prelude::*;
use noise::{Fbm, NoiseFn, Perlin, Seedable};

use crate::game::trauma::Trauma;

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ShakyMovementPlugin;

impl Plugin for ShakyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Components
////////////////////////////////////////////////////////////////////////////////

///
/// ## Notes
/// Uses perlin noise to generate a smooth random number. This makes the camera
/// shake smoothly during slow motion. (NOT TESTED)
///
#[derive(Component)]
pub struct ShakyMovement {
    pub trauma_entity: Entity,
    pub transform_entity: Entity,
    pub max_angel: f32,
    pub max_offset: f32,
    pub angel_perlin: Fbm<Perlin>,
    pub x_perlin: Fbm<Perlin>,
    pub y_perlin: Fbm<Perlin>,
}

const MAX_ANGEL: f32 = 0.05;
const MAX_SHAKY_OFFSET: f32 = 4.0;

impl ShakyMovement {
    pub fn basic(trauma_entity: Entity, transform_entity: Entity) -> Self {
        let fbm = Fbm::<Perlin>::default();

        Self {
            trauma_entity,
            transform_entity,
            max_angel: MAX_ANGEL,
            max_offset: MAX_SHAKY_OFFSET,
            angel_perlin: fbm.clone().set_seed(124135),
            x_perlin: fbm.clone().set_seed(123),
            y_perlin: fbm.clone().set_seed(43212),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

fn update(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ShakyMovement)>,
    mut trasform_query: Query<&Transform, Without<ShakyMovement>>,
    trauma_query: Query<&Trauma, Without<ShakyMovement>>,
) {
    for (mut movement_transform, movement) in query.iter_mut() {
        if let Ok(target_transform) = trasform_query.get_mut(movement.transform_entity) {
            if let Ok(target_trauma) = trauma_query.get(movement.trauma_entity) {
                let shake = target_trauma.get_trauma().powi(2);

                // The sample point must be relative to time such that
                // slowmotion slows down the shaking. (NOT TESTED)
                let sample_point = [time.elapsed_seconds_f64(), time.elapsed_seconds_f64()];

                // Get a random numbers between -1 and 1
                let random_angel =
                    movement.max_angel * shake * (movement.angel_perlin.get(sample_point) as f32);
                let random_offset_x =
                    movement.max_offset * shake * (movement.x_perlin.get(sample_point) as f32);
                let random_offset_y =
                    movement.max_offset * shake * (movement.y_perlin.get(sample_point) as f32);

                // Add the random numbers to the target transform
                movement_transform.translation =
                    target_transform.translation + Vec3::new(random_offset_x, random_offset_y, 0.0);
                movement_transform.rotation = target_transform
                    .rotation
                    .mul_quat(Quat::from_rotation_z(random_angel));
                movement_transform.scale = target_transform.scale;
            } else {
                movement_transform.translation = target_transform.translation;
                movement_transform.rotation = target_transform.rotation;
                movement_transform.scale = target_transform.scale;
            }
        }
    }
}
