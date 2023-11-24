use bevy::prelude::*;

use super::{GameScene, SceneEntityLabel};
use crate::game::{arena, background, enemy, movement::FollowEntityMovement, player_camera};

pub struct EnemyShipAIScenePlugin;

// TODO: Move the Arena code to the main_game.rs file
impl Plugin for EnemyShipAIScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::EnemyShipAI), (background::spawn, spawn))
            .add_systems(
                OnExit(GameScene::EnemyShipAI),
                (
                    background::despawn,
                    player_camera::despawn,
                    arena::despawn,
                    enemy::despawn,
                    SceneEntityLabel::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the arena and player
    let arena = arena::Arena::new(1000.0, 200.0);
    arena.spawn_asteroid_bounds(&mut commands, &asset_server);
    arena.spawn_random_asteroids(&mut commands, &asset_server, 50);
    // arena.spawn_player(&mut commands, &asset_db, &asset_server);

    // Spawn an enemy ship
    let enemy_entity = enemy::spawn(&mut commands, &asset_server, Vec2::new(0.0, 0.0), 0.0);

    commands
        .spawn(Camera2dBundle::default())
        .insert(FollowEntityMovement::smooth(enemy_entity))
        .insert(SceneEntityLabel);
}
