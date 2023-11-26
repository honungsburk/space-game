use bevy::prelude::*;

use super::GameScene;
use crate::{
    game::{
        arena, background, kamikaze_drone,
        movement::FollowEntityMovement,
        player_camera::{self},
    },
    misc::random,
    utility_systems::cleanup,
};

pub struct KamikazeDroneScenePlugin;

impl Plugin for KamikazeDroneScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::KamikazeDrone),
            (background::spawn, spawn),
        )
        .add_systems(
            OnExit(GameScene::KamikazeDrone),
            (
                background::despawn,
                player_camera::despawn,
                arena::despawn,
                kamikaze_drone::despawn,
                cleanup::<Camera>,
            ),
        );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the arena and player
    let radius = 1000.0;
    let arena = arena::Arena::new(radius, 200.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_server);
    arena.spawn_random_asteroids(&mut commands, &asset_server, 50);

    // Spawn an enemy ship
    let kamikaze_drone_entity =
        kamikaze_drone::spawn(&mut commands, &asset_server, Vec2::new(0.0, 0.0), 0.0);

    commands
        .spawn(Camera2dBundle::default())
        .insert(FollowEntityMovement::smooth(kamikaze_drone_entity));

    let spawn_count = 100;

    let mut rng = rand::thread_rng();

    for _ in 0..spawn_count {
        let point_in_circle = random::uniform_circle(&mut rng, radius * 0.5);

        kamikaze_drone::spawn(
            &mut commands,
            &asset_server,
            point_in_circle,
            rand::random::<f32>() * std::f32::consts::PI * 2.0,
        );
    }

    // kamikaze_drone::spawn(&mut commands, &asset_server, Vec2::new(40.0, 0.0), 0.0);

    // kamikaze_drone::spawn(&mut commands, &asset_server, Vec2::new(80.0, 0.0), 0.0);
}
