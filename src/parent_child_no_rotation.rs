use bevy::{prelude::*, transform::TransformSystem};

pub struct NoRotationPlugin;

impl Plugin for NoRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            update_fransform_no_rotation.before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            update_fransform_no_rotation.before(TransformSystem::TransformPropagate),
        );
    }
}

// Placed on the child entity, this component will cause the child to have its transform
// updated to match the parent entity's transform. But it will not inherit the parent's
// rotation.
#[derive(Component)]
pub struct NoRotationChild;

// Placed on the parent entity, this component will cause the parent to have its transform
// updated to match the child entity's transform. But it will not inherit the child's
// rotation.
#[derive(Component)]
pub struct NoRotationParent;

/// there is no way to inherit position but not rotation from the parent entity transform yet
/// see: https://github.com/bevyengine/bevy/issues/1780
/// so labels will rotate with ships unless we fiddle with it:
/// TODO: remove this when the issue is fixed
/// TODO: There seems to be a bug with this system, where the child will rotate for a split second
/// before the rotation is set to 0.0.
pub fn update_fransform_no_rotation(
    mut q_text: Query<(&Parent, &mut Transform), With<NoRotationChild>>,
    q_parents: Query<&Transform, (With<NoRotationParent>, Without<NoRotationChild>)>,
) {
    for (parent, mut transform) in q_text.iter_mut() {
        if let Ok(parent_transform) = q_parents.get(parent.get()) {
            // global transform propagation system will make the rotation 0 now
            transform.rotation = parent_transform.rotation.inverse();
        }
    }
}
