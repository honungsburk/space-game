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
            .add_systems(Update, (update_background, update_tile_debug));
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Component
////////////////////////////////////////////////////////////////////////////////

// The background that the player will see. It's just a sprite that's
// scaled to the size of the window, and shows distant stars.
#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct BackgroundCenterTile;

#[derive(Component)]
pub struct BackgroundTile {
    pub x: i32,
    pub y: i32,
}

////////////////////////////////////////////////////////////////////////////////
/// System
////////////////////////////////////////////////////////////////////////////////

const BACKGROUND_TILE_WIDTH: f32 = 256.0;
const BACKGROUND_TILE_HEIGHT: f32 = 256.0;
const BACKGROUND_TILES_SIZE: u32 = 3;

// Spawn the background
pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // let window = window_query.get_single().unwrap();
    // let window_width = window.width();
    // let window_height = window.height();

    // let mut transform = Transform::from_xyz(
    //     0.0, 0.0, -1.0, // The z axis is used to place the background behind everything
    // );

    // transform.scale = Vec3::new(window_width / 256.0, window_height / 256.0, 1.0);

    // commands

    //     .spawn(SpriteBundle {
    //         transform: transform,
    //         texture: asset_server.load("sprites/backgrounds/black.png"),
    //         ..Default::default()
    //     })
    //     .insert(Background);

    let background_handle = asset_server.load("sprites/backgrounds/black.png");

    // We will spawn 9 background sprites, one for each of the 9
    // positions on the screen. This will allow us to scroll the
    // background in any direction.

    let low_bound = -1 * (BACKGROUND_TILES_SIZE as i32) / 2;
    let high_bound = (BACKGROUND_TILES_SIZE as i32) / 2;

    println!("low_bound: {}", low_bound);
    println!("high_bound: {}", high_bound);

    for x in low_bound..=high_bound {
        for y in low_bound..=high_bound {
            let position = background_tile_position(x, y);
            let transform = Transform::from_xyz(
                position.x, position.y,
                -1.0, // The z axis is used to place the background behind everything
            );

            // transform.scale = Vec3::new(
            //     window_width / (BACKGROUND_WIDTH * BACKGROUND_TILE_WIDTH),
            //     window_height / (BACKGROUND_HEIGHT * BACKGROUND_TILE_HEIGHT),
            //     1.0,
            // );

            commands
                .spawn(SpriteBundle {
                    transform: transform,
                    texture: background_handle.clone(),
                    ..Default::default()
                })
                .insert(BackgroundTile { x, y });
        }
    }
}

fn update_tile_debug(mut gizmos: Gizmos, mut query: Query<&Transform, With<BackgroundTile>>) {
    for transform in query.iter_mut() {
        gizmos.rect_2d(
            transform.translation.truncate(),
            0.,
            Vec2::new(BACKGROUND_TILE_WIDTH, BACKGROUND_TILE_HEIGHT),
            Color::WHITE,
        );
    }
}

fn background_tile_position(x: i32, y: i32) -> Vec2 {
    Vec2::new(
        x as f32 * BACKGROUND_TILE_WIDTH,
        y as f32 * BACKGROUND_TILE_HEIGHT,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_tile_position() {
        let pos = background_tile_position(0, 0);
        assert_eq!(pos, Vec2::new(0.0, 0.0));

        let pos = background_tile_position(1, 1);
        assert_eq!(
            pos,
            Vec2::new(BACKGROUND_TILE_WIDTH, BACKGROUND_TILE_HEIGHT)
        );

        let pos = background_tile_position(-2, 3);
        assert_eq!(
            pos,
            Vec2::new(BACKGROUND_TILE_WIDTH * -2.0, BACKGROUND_TILE_HEIGHT * 3.0)
        );
    }
}

// Imagine the entire world is a grid of tiles. The tile the player is
// currently on is the center tile. This function returns the center
// tile's position.
fn background_center(camera_position: Vec3) -> Vec2 {
    let x = (camera_position.x / BACKGROUND_TILE_WIDTH).floor() as i32;
    let y = (camera_position.y / BACKGROUND_TILE_HEIGHT).floor() as i32;

    background_tile_position(x, y)
}

// The background follows the camera, but at a slower rate
pub fn update_background(
    // window_query: Query<&Window, With<PrimaryWindow>>,
    mut query_background: Query<(&mut Transform, &BackgroundTile), Without<ShakyCamera>>,
    query_camera: Query<&GlobalTransform, (Without<BackgroundTile>, With<ShakyCamera>)>,
) {
    // let window = window_query.get_single().unwrap();
    // let window_width = window.width();
    // let window_height = window.height();

    if let Ok(camera_transform) = query_camera.get_single() {
        let tile_center = background_center(camera_transform.translation());

        for (mut transform, background_tile) in query_background.iter_mut() {
            let relative_tile_position =
                background_tile_position(background_tile.x, background_tile.y);
            let absolute_tile_position = tile_center + relative_tile_position;

            transform.translation.x = absolute_tile_position.x;
            transform.translation.y = absolute_tile_position.y;
            transform.translation.z = -1.0;
        }
    }
}
