mod collider;
mod target;
mod targets;

use bevy::prelude::*;

pub use collider::{ColliderSensorBundle, ColliderSensorLabel};
pub use target::{SensorTarget, SensorTargetVec2};
pub use targets::SensorTargets;

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (collider::update_targets, target::update_vec2_target),
        );
    }
}
