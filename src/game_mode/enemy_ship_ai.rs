use bevy::prelude::*;

use super::GameMode;
use crate::game::{arena, assets::AssetDB, background, camera, enemy};

pub struct EnemyShipAIPlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for EnemyShipAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMode::EnemyShipAI),
            (background::spawn, camera::spawn, spawn),
        )
        .add_systems(
            OnExit(GameMode::EnemyShipAI),
            (
                background::despawn,
                camera::despawn,
                arena::despawn,
                enemy::despawn,
            ),
        );
    }
}

fn spawn(mut commands: Commands, asset_db: Res<AssetDB>, asset_server: Res<AssetServer>) {
    // Spawn the arena and player
    let arena = arena::Arena::new(500.0, 200.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_db, &asset_server);
    // arena.spawn_random_asteroids(&mut commands, &asset_db, &asset_server, 100);
    // arena.spawn_player(&mut commands, &asset_db, &asset_server);

    // Spawn an enemy ship
    let enemy_entity = enemy::spawn(
        &mut commands,
        &asset_db,
        &asset_server,
        Vec2::new(0.0, 0.0),
        0.0,
    );
}
