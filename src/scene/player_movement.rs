use bevy::prelude::*;

use super::GameScene;
use crate::game::{arena, background, player, player_camera};

pub struct PlayerMovementScenePlugin;

impl Plugin for PlayerMovementScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(
                OnEnter(GameScene::PlayerMovement),
                (background::spawn, spawn),
            )
            .add_systems(
                OnExit(GameScene::PlayerMovement),
                (
                    player::despawn,
                    background::despawn,
                    player_camera::despawn,
                    arena::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let arena = arena::Arena::new(1000.0, 200.0);
    arena.spawn_random_asteroids(&mut commands, &asset_server, 50);

    let player_entity =
        player::spawn_player(&mut commands, &asset_server, Vec2::new(0.0, 0.0), 0.0);

    player_camera::spawn(&mut commands, player_entity);
}
