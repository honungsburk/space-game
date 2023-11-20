use bevy::app::AppExit;
use bevy::prelude::*;

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

/// Cleanup all entities with a specific component.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
///
/// #[derive(Component)]
/// struct SomeLabel;
///
/// struct SomePlugin;
///
/// impl Plugin for SomePlugin {
/// fn build(&self, app: &mut App) {
///    app.add_systems(OnEnter(Scene::SomeScene), spawn)
///        .add_systems(
///            OnExit(Scene::SomeScene),
///            utility_systems::cleanup::<SomeLabel>,
///        );
///  }
///}
///
/// fn spawn(mut commands: Commands) {
///    commands.spawn((SomeLabel, Transform::default()));
/// }
///
///
pub fn cleanup<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
