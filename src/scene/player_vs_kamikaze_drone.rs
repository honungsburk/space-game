use bevy::prelude::*;

use super::GameScene;
use crate::{
    game::{
        arena, background,
        guard_point::GuardPoint,
        kamikaze_drone,
        player_camera::{self},
    },
    misc::random,
    utility_systems::cleanup,
};

pub struct PlayerVsKamikazeDroneScenePlugin;

impl Plugin for PlayerVsKamikazeDroneScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::PlayerVsKamikazeDrone),
            (background::spawn, spawn),
        )
        .add_systems(
            OnExit(GameScene::PlayerVsKamikazeDrone),
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
    let radius = 2000.0;
    let arena = arena::Arena::new(radius, 300.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_server);
    arena.spawn_random_asteroids(&mut commands, &asset_server, 50);
    arena.spawn_player(&mut commands, &asset_server);

    // Spawn enemy drones

    let guard_point = GuardPoint::new(Vec2::new(0.0, 0.0), radius);

    let spawn_count = 100;

    let mut rng = rand::thread_rng();

    for _ in 0..spawn_count {
        let point_in_circle = random::uniform_donut(&mut rng, radius * 0.9, radius * 0.5);

        kamikaze_drone::spawn(
            &mut commands,
            &asset_server,
            point_in_circle,
            rand::random::<f32>() * std::f32::consts::PI * 2.0,
            Some(guard_point),
        );
    }
}
