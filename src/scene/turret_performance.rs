use bevy::prelude::*;

use super::GameScene;
use crate::game::{background, camera, player, turret};

pub struct TurretPerformanceScenePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for TurretPerformanceScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(
                OnEnter(GameScene::TurretPerformance),
                (
                    player::spawn(Vec2::new(0.0, 0.0), 0.0),
                    background::spawn,
                    camera::spawn,
                    spawn,
                ),
            )
            .add_systems(
                OnExit(GameScene::TurretPerformance),
                (
                    player::despawn,
                    background::despawn,
                    camera::despawn,
                    turret::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn turrets in a grid north of the player

    let turret_count = 1000;

    let turret_spacing = 100.0;

    let turret_grid_size = (turret_count as f32).sqrt().ceil() as u32;

    for x in 0..turret_grid_size {
        for y in 0..turret_grid_size {
            let turret_location = Vec2::new(x as f32 * turret_spacing, y as f32 * turret_spacing)
                + Vec2::new(200.0, 0.0);

            let spawn_transform = Transform::from_translation(turret_location.extend(0.0));

            turret::spawn(
                &mut commands,
                &asset_server,
                &turret::TurretConfig::default(),
                spawn_transform,
            )
        }
    }
}
