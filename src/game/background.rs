use bevy::{prelude::*, window::PrimaryWindow};

use super::camera::ShakyCamera;

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

// The plugin that spawns the background
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
            .add_systems(Update, update_background);
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Component
////////////////////////////////////////////////////////////////////////////////

// The background that the player will see. It's just a sprite that's
// scaled to the size of the window, and shows distant stars.
#[derive(Component)]
pub struct Background;

////////////////////////////////////////////////////////////////////////////////
/// System
////////////////////////////////////////////////////////////////////////////////

// Spawn the background
pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_scale(Vec3::new(
                window_width / 16.0,
                window_height / 16.0,
                1.0, // The z axis is used to place the background behind everything
            )),
            texture: asset_server.load("sprites/backgrounds/black.png"),
            ..Default::default()
        })
        .insert(Background);
}

// The background follows the camera, but at a slower rate
pub fn update_background(
    // window_query: Query<&Window, With<PrimaryWindow>>,
    mut query_background: Query<&mut Transform, (With<Background>, Without<ShakyCamera>)>,
    query_camera: Query<&Transform, (Without<Background>, With<ShakyCamera>)>,
) {
    // let window = window_query.get_single().unwrap();
    // let window_width = window.width();
    // let window_height = window.height();

    if let Ok(camera_transform) = query_camera.get_single() {
        for mut transform in query_background.iter_mut() {
            transform.translation.x = camera_transform.translation.x;
            transform.translation.y = camera_transform.translation.y;
            transform.translation.z = 1.0;
        }
    }
}
