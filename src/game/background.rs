use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use super::camera::ShakyCamera;
use super::config::Flag;

const BACKGROUND_TILE_WIDTH: f32 = 256.0;
const BACKGROUND_TILE_HEIGHT: f32 = 256.0;
const BACKGROUND_TILE_SCALE: f32 = 2.0;

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

// The plugin that spawns the background
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundGrid {
            grid: Grid::new(
                BACKGROUND_TILE_WIDTH * BACKGROUND_TILE_SCALE,
                BACKGROUND_TILE_HEIGHT * BACKGROUND_TILE_SCALE,
            ),
        })
        .init_resource::<BackgroundGridDebugFlag>()
        .add_systems(Startup, spawn_background)
        .add_systems(
            Update,
            (
                update_background,
                on_window_resize,
                update_background_grid_debug.run_if(condition_debug_background),
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Components & Resources
////////////////////////////////////////////////////////////////////////////////

#[derive(Resource)]
struct BackgroundGrid {
    grid: Grid,
}

/// A resource that controls if the background grid lines should be shown
#[derive(Resource, Debug)]
pub struct BackgroundGridDebugFlag {
    pub flag: Flag,
}

impl Default for BackgroundGridDebugFlag {
    fn default() -> Self {
        Self {
            flag: Flag::new("Background Grid", "Display the background grid", false),
        }
    }
}
#[derive(Component)]
struct BackgroundTile {
    pub tile: Tile,
}

////////////////////////////////////////////////////////////////////////////////
/// Run conditions
////////////////////////////////////////////////////////////////////////////////

fn condition_debug_background(background_grid_debug: Res<BackgroundGridDebugFlag>) -> bool {
    background_grid_debug.flag.is_on()
}

////////////////////////////////////////////////////////////////////////////////
/// System
////////////////////////////////////////////////////////////////////////////////

// Spawn the background
fn spawn_background(
    mut commands: Commands,
    background_grid: Res<BackgroundGrid>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    spawn_background_tiles(
        &mut commands,
        &background_grid,
        (window.width(), window.height()),
        &asset_server,
    )
}

fn on_window_resize(
    mut commands: Commands,
    background_grid: Res<BackgroundGrid>,
    mut resize_reader: EventReader<WindowResized>,
    background_tile_query: Query<Entity, With<BackgroundTile>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(e) = resize_reader.iter().last() {
        // Remove all background tiles
        for entity in background_tile_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        spawn_background_tiles(
            &mut commands,
            &background_grid,
            (e.width, e.height),
            &asset_server,
        );
    }
}

fn spawn_background_tiles(
    commands: &mut Commands,
    background_grid: &Res<BackgroundGrid>,
    window_resolution: (f32, f32),
    asset_server: &Res<AssetServer>,
) {
    let background_handle = asset_server.load("sprites/backgrounds/blue.png");

    let (x_res, y_res) = window_resolution;

    let (x_tiles, y_tiles) = background_grid.grid.number_of_tiles(x_res, y_res);

    let (low_bound_x, high_bound_x) = make_bounds(x_tiles);
    let (low_bound_y, high_bound_y) = make_bounds(y_tiles);

    for x in low_bound_x..=high_bound_x {
        for y in low_bound_y..=high_bound_y {
            let mut transform = Transform::from_xyz(
                0.0, 0.0, -1.0, // The z axis is used to place the background behind everything
            );

            transform.scale = Vec3::new(BACKGROUND_TILE_SCALE, BACKGROUND_TILE_SCALE, 1.0);

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

fn update_background_grid_debug(
    background_grid: Res<BackgroundGrid>,
    mut gizmos: Gizmos,
    mut query: Query<&Transform, With<BackgroundTile>>,
) {
    for transform in query.iter_mut() {
        gizmos.rect_2d(
            transform.translation.truncate(),
            0.,
            Vec2::new(
                background_grid.grid.tile_width,
                background_grid.grid.tile_height,
            ),
            Color::WHITE,
        );
    }
}

// Move the background tiles to follow the camera.
// The center background tile is always in the same tile as the camera.
fn update_background(
    background_grid: Res<BackgroundGrid>,
    mut query_background: Query<(&mut Transform, &BackgroundTile), Without<ShakyCamera>>,
    query_camera: Query<&GlobalTransform, (Without<BackgroundTile>, With<ShakyCamera>)>,
) {
    if let Ok(camera_transform) = query_camera.get_single() {
        let camera_tile = background_grid
            .grid
            .tile(camera_transform.translation().truncate());
        let tile_center = background_grid.grid.tile_position(camera_tile);

        for (mut transform, background_tile) in query_background.iter_mut() {
            let relative_tile_position = background_grid.grid.tile_position(background_tile.tile);
            let absolute_tile_position = tile_center + relative_tile_position;

            transform.translation.x =
                absolute_tile_position.x + background_grid.grid.tile_width / 2.0;
            transform.translation.y =
                absolute_tile_position.y + background_grid.grid.tile_height / 2.0;
            transform.translation.z = -1.0;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Helpers
///////////////////////////////////////////////////////////////////////////////

fn make_bounds(n: u32) -> (i32, i32) {
    let low_bound = -1 * (n as i32) / 2;
    let high_bound = (n as i32) / 2;

    (low_bound, high_bound)
}

////////////////////////////////////////////////////////////////////////////////
/// Background Grid
///////////////////////////////////////////////////////////////////////////////

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
    tile_width: f32,
    tile_height: f32,
}

impl Grid {
    fn new(tile_width: f32, tile_height: f32) -> Self {
        Self {
            tile_width,
            tile_height,
        }
    }

    fn center(&self) -> Tile {
        Tile::default()
    }

    /// Return the number of tiles in x and y directions that are needed to cover the screen.
    /// The numbers are allways odd.
    fn number_of_tiles(&self, screen_width: f32, screen_height: f32) -> (u32, u32) {
        let number_of_tiles_x = (screen_width / self.tile_width).ceil() as u32;
        let number_of_tiles_y = (screen_height / self.tile_height).ceil() as u32;

        // Make sure the number of tiles is odd
        let number_of_tiles_x = if number_of_tiles_x % 2 == 0 {
            number_of_tiles_x + 1
        } else {
            number_of_tiles_x
        };

        let number_of_tiles_y = if number_of_tiles_y % 2 == 0 {
            number_of_tiles_y + 1
        } else {
            number_of_tiles_y
        };

        // To avoid seeing the edge of the background, we add one tile on each side
        (number_of_tiles_x + 2, number_of_tiles_y + 2)
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
        let grid = Grid::new(10.0, 10.0);
        let pos = grid.tile_position_xy(0, 0);
        assert_eq!(pos, Vec2::new(0.0, 0.0));

        let pos = grid.tile_position_xy(1, 1);
        assert_eq!(pos, Vec2::new(10.0, 10.0));

        let pos = grid.tile_position_xy(-2, 3);
        assert_eq!(pos, Vec2::new(-20.0, 30.0));
    }

    #[test]
    fn test_background_tile() {
        let grid = Grid::new(10.0, 10.0);
        let camera_position = Vec2::new(0.0, 0.0);
        assert_eq!(grid.tile(camera_position), Tile::new(0, 0));

        let camera_position = Vec2::new(0.0, 30.0);
        assert_eq!(grid.tile(camera_position), Tile::new(0, 3));

        let camera_position = Vec2::new(25.0, 0.0);
        assert_eq!(grid.tile(camera_position), Tile::new(2, 0));

        let camera_position = Vec2::new(-50.0, -50.0);
        assert_eq!(grid.tile(camera_position), Tile::new(-5, -5));
    }
}
