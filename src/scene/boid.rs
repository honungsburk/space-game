use bevy::{prelude::*, window::PrimaryWindow};

use super::GameScene;
use crate::game::{
    arena, background, boids,
    player_camera::{self},
};
use crate::utility_systems::cleanup;

pub struct BoidScenePlugin;

impl Plugin for BoidScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Boid), (background::spawn, spawn))
            .add_systems(
                OnExit(GameScene::Boid),
                (
                    background::despawn,
                    cleanup::<Camera>,
                    arena::despawn,
                    boids::despawn,
                ),
            );
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    // Spawn the arena and player
    let arena = arena::Arena::new(1000.0, 200.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_server);
    // arena.spawn_random_asteroids(&mut commands, &asset_db, &asset_server, 50);
    // Spawn 100 boids at random locations
    let boid_count = 100;

    for _ in 0..boid_count {
        boids::spawn(
            &mut commands,
            &asset_server,
            Vec2::new(
                (rand::random::<f32>() - 0.5) * window.width(),
                (rand::random::<f32>() - 0.5) * window.height(),
            ),
            rand::random::<f32>() * std::f32::consts::PI * 2.0,
        );
    }

    commands.spawn(Camera2dBundle::default());
}
