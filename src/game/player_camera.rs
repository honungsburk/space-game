use super::movement::{FollowEntityMovementBundle, ShakyMovement};
use bevy::prelude::*;

/// Label for the player camera
#[derive(Component)]
pub struct PlayerCameraLabel;

/// The player camera is constructed from two cameras:
/// - A smooth camera that follows the player
/// - A shaky camera that is what the player sees
///
/// The smooth camera is offset from the player by a PID controller.
/// The shaky camera is offset from the smooth camera by a random amount.
pub fn spawn(commands: &mut Commands, target: Entity) -> (Entity, Entity) {
    let transform = Transform::from_xyz(0.0, 0.0, 0.0);

    // Smooth camera
    let smooth_camera_id = commands
        .spawn(FollowEntityMovementBundle::smooth(target))
        .insert(PlayerCameraLabel)
        .id();

    // Shaky camera (What the player sees)
    let shaky_camera_id = commands
        .spawn(Camera2dBundle::default())
        .insert(ShakyMovement::basic(target, smooth_camera_id))
        .insert(PlayerCameraLabel)
        .id();

    (smooth_camera_id, shaky_camera_id)
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<PlayerCameraLabel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
