use bevy::{prelude::*, window::PrimaryWindow};

use super::camera::ShakyCamera;

const BACKGROUND_TILE_WIDTH: f32 = 256.0;
const BACKGROUND_TILE_HEIGHT: f32 = 256.0;
const BACKGROUND_TILES_SIZE: u32 = 3;

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

// The plugin that spawns the background
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundGrid {
            grid: Grid::new(
                BACKGROUND_TILES_SIZE,
                BACKGROUND_TILE_WIDTH,
                BACKGROUND_TILE_HEIGHT,
            ),
        })
        .add_systems(Startup, spawn_background)
        .add_systems(Update, (update_background, update_tile_debug));
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Component
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct BackgroundTile {
    pub tile: Tile,
}

////////////////////////////////////////////////////////////////////////////////
/// System
////////////////////////////////////////////////////////////////////////////////

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
            let transform = Transform::from_xyz(
                0.0, 0.0, -1.0, // The z axis is used to place the background behind everything
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
                .insert(BackgroundTile {
                    tile: Tile::new(x, y),
                });
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

// The background follows the camera, but at a slower rate
fn update_background(
    // window_query: Query<&Window, With<PrimaryWindow>>,
    background_grid: Res<BackgroundGrid>,
    mut query_background: Query<(&mut Transform, &BackgroundTile), Without<ShakyCamera>>,
    query_camera: Query<&GlobalTransform, (Without<BackgroundTile>, With<ShakyCamera>)>,
) {
    // let window = window_query.get_single().unwrap();
    // let window_width = window.width();
    // let window_height = window.height();

    if let Ok(camera_transform) = query_camera.get_single() {
        let tile = background_grid
            .grid
            .tile(camera_transform.translation().truncate());
        let tile_center = background_grid.grid.tile_position(tile);

        for (mut transform, background_tile) in query_background.iter_mut() {
            let relative_tile_position = background_grid.grid.tile_position(background_tile.tile);
            let absolute_tile_position = tile_center + relative_tile_position;

            transform.translation.x = absolute_tile_position.x;
            transform.translation.y = absolute_tile_position.y;
            transform.translation.z = -1.0;
        }
    }
}
////////////////////////////////////////////////////////////////////////////////
/// Background Grid
///////////////////////////////////////////////////////////////////////////////

#[derive(Resource)]
struct BackgroundGrid {
    grid: Grid,
}

// A tile in the background grid

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    x: i32, // x position in the grid
    y: i32, // y position in the grid
}

impl Tile {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// The background grid
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    size: u32, // number of tiles in each direction, so a size of 3 means 3x3 tiles. Must be odd.
    tile_width: f32,
    tile_height: f32,
}

impl Grid {
    fn new(size: u32, tile_width: f32, tile_height: f32) -> Self {
        assert!(size % 2 == 1);
        Self {
            size,
            tile_width,
            tile_height,
        }
    }

    fn center(&self) -> Tile {
        Tile::default()
    }

    fn tile_position(&self, tile: Tile) -> Vec2 {
        self.tile_position_xy(tile.x, tile.y)
    }

    fn tile_position_xy(&self, x: i32, y: i32) -> Vec2 {
        Vec2::new(x as f32 * self.tile_width, y as f32 * self.tile_height)
    }

    /// Returns the tile that the position is in
    fn tile(&self, position: Vec2) -> Tile {
        self.tile_xy(position.x, position.y)
    }

    fn tile_xy(&self, x: f32, y: f32) -> Tile {
        let x = (x / self.tile_width).floor() as i32;
        let y = (y / self.tile_height).floor() as i32;

        Tile::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_position() {
        let grid = Grid::new(3, 10.0, 10.0);
        let pos = grid.tile_position_xy(0, 0);
        assert_eq!(pos, Vec2::new(0.0, 0.0));

        let pos = grid.tile_position_xy(1, 1);
        assert_eq!(pos, Vec2::new(10.0, 10.0));

        let pos = grid.tile_position_xy(-2, 3);
        assert_eq!(pos, Vec2::new(-20.0, 30.0));
    }

    #[test]
    fn test_background_tile() {
        let grid = Grid::new(3, 10.0, 10.0);
        let camera_position = Vec2::new(0.0, 0.0);
        assert_eq!(grid.tile(camera_position), Tile::new(0, 0));

        let camera_position = Vec2::new(100.0, 100.0);
        assert_eq!(grid.tile(camera_position), Tile::new(10, 10));

        let camera_position = Vec2::new(-50.0, -50.0);
        assert_eq!(grid.tile(camera_position), Tile::new(-5, -5));
    }
}
