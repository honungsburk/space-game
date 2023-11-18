use bevy::prelude::*;

use super::Scene;
use crate::game::{
    arena,
    assets::AssetDB,
    background, boids,
    camera::{self, CameraTargetLabel},
};

pub struct BoidScenePlugin;

impl Plugin for BoidScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Scene::Boid),
            (background::spawn, camera::spawn, spawn),
        )
        .add_systems(
            OnExit(Scene::Boid),
            (
                background::despawn,
                camera::despawn,
                arena::despawn,
                boids::despawn,
            ),
        );
    }
}

fn spawn(mut commands: Commands, asset_db: Res<AssetDB>, asset_server: Res<AssetServer>) {
    // Spawn the arena and player
    let arena = arena::Arena::new(1000.0, 200.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_db, &asset_server);
    // arena.spawn_random_asteroids(&mut commands, &asset_db, &asset_server, 50);

    // Spawn an enemy ship
    // let kamikaze_drone_entity = boids::spawn(
    //     &mut commands,
    //     &asset_db,
    //     &asset_server,
    //     Vec2::new(0.0, 0.0),
    //     0.0,
    // );

    // commands
    //     .entity(kamikaze_drone_entity)
    //     .insert(CameraTargetLabel);

    // spawn 99 more drones

    boids::spawn(
        &mut commands,
        &asset_db,
        &asset_server,
        Vec2::new(40.0, 0.0),
        0.0,
    );

    boids::spawn(
        &mut commands,
        &asset_db,
        &asset_server,
        Vec2::new(80.0, 0.0),
        0.0,
    );

    // let drone_count = 1;

    // for _ in 0..drone_count {
    //     kamikaze_drone::spawn(
    //         &mut commands,
    //         &asset_db,
    //         &asset_server,
    //         Vec2::new(10.0 * drone_count as f32, 0.0),
    //         0.0,
    //     );
    // }
}
