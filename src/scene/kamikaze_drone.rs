use bevy::prelude::*;

use super::Scene;
use crate::game::{
    arena,
    assets::AssetDB,
    background,
    camera::{self, CameraTargetLabel},
    kamikaze_drone,
};

pub struct KamikazeDroneScenePlugin;

impl Plugin for KamikazeDroneScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Scene::KamikazeDrone),
            (background::spawn, camera::spawn, spawn),
        )
        .add_systems(
            OnExit(Scene::KamikazeDrone),
            (
                background::despawn,
                camera::despawn,
                arena::despawn,
                kamikaze_drone::despawn,
            ),
        );
    }
}

fn spawn(mut commands: Commands, asset_db: Res<AssetDB>, asset_server: Res<AssetServer>) {
    // Spawn the arena and player
    let arena = arena::Arena::new(1000.0, 200.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_db, &asset_server);
    arena.spawn_random_asteroids(&mut commands, &asset_db, &asset_server, 50);

    // Spawn an enemy ship
    let kamikaze_drone_entity = kamikaze_drone::spawn(
        &mut commands,
        &asset_db,
        &asset_server,
        Vec2::new(0.0, 0.0),
        0.0,
    );

    commands
        .entity(kamikaze_drone_entity)
        .insert(CameraTargetLabel);
}
