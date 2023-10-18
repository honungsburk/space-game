use bevy_rapier2d::geometry::Group;

use bevy::prelude::*;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurretAI {
    state: TurretState,
    target_group: Group,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurretState {
    Idle,
    Targeting,
    Firing,
}

impl Default for TurretState {
    fn default() -> Self {
        Self::Idle
    }
}

impl TurretState {
    // pub fn update(
    //     &mut self,
    //     target: Option<Entity>,
    //     transform: &Transform,
    //     target_transform: &Transform,
    //     commands: &mut Commands,
    //     asset_db: &Res<AssetDB>,
    //     asset_server: &Res<AssetServer>,
    //     collision_membership: &Group,
    //     collision_filter: &Group,
    // ) {
    //     match self {
    //         Self::Idle => {
    //             if let Some(target) = target {
    //                 *self = Self::Targeting;
    //             }
    //         }
    //         Self::Targeting => {
    //             if let Some(target) = target {
    //                 let target_transform = target_transform.translation;
    //                 let turret_transform = transform.translation;

    //                 let target_direction = target_transform - turret_transform;
    //                 let target_direction = target_direction.normalize();

    //                 let turret_direction = transform.rotation.mul_vec3(Vec3::Y);

    //                 let angle = turret_direction.angle_between(target_direction);

    //                 if angle < turret.targeting_angle {
    //                     *self = Self::Firing;
    //                 }
    //             } else {
    //                 *self = Self::Idle;
    //             }
    //         }
    //         Self::Firing => {
    //             if let Some(target) = target {
    //                 let target_transform = target_transform.translation;
    //                 let turret_transform = transform.translation;

    //                 let target_direction = target_transform - turret_transform;
    //                 let target_direction = target_direction.normalize();

    //                 let turret_direction = transform.rotation.mul_vec3(Vec3::Y);

    //                 let angle = turret_direction.angle_between(target_direction);

    //                 if angle > turret.targeting_angle {
    //                     *self = Self::Targeting;
    //                 } else {
    //                     projectile_spawner.spawn_projectile(
    //                         commands,
    //                         asset_db,
    //                         asset_server,
    //                         transform.clone(),
    //                         collision_membership,
    //                         collision_filter,
    //                     );
    //                 }
    //             } else {
    //                 *self = Self::Idle;
    //             }
    //         }
    //     }
    // }
}
