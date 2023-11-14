use crate::misc::rapier_extension;

use super::components::KamikazeDroneLabel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

const KAMIKAZE_DRONE_SPEED: f32 = 100.0;

pub fn update_kamikaze_drone(
    gizmos: Gizmos,
    time: Res<Time>,
    rapier_ctx: Res<RapierContext>,
    mut kamikaze_query: Query<&mut Transform, With<KamikazeDroneLabel>>,
) {
    let filter = QueryFilter::default();

    let mut giz = Some(gizmos);

    for mut kamikaze_transform in kamikaze_query.iter_mut() {
        if let Some(path) = rapier_extension::find_unobstructed_path(
            &rapier_ctx,
            &mut giz,
            &kamikaze_transform,
            1.0,
            10.0,
            200.0,
            10.0,
            filter,
            None,
        ) {
            kamikaze_transform.translation +=
                path.extend(0.0) * KAMIKAZE_DRONE_SPEED * time.delta_seconds();
            kamikaze_transform.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(path));
        } else {
            let rotation = kamikaze_transform.rotation;
            kamikaze_transform.translation +=
                rotation.mul_vec3(Vec3::Y * KAMIKAZE_DRONE_SPEED * time.delta_seconds());
        }
    }
}
