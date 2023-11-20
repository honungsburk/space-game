use bevy::prelude::*;

use super::Scene;
use crate::game::{arena, background, camera, player};

pub struct PlayerMovementScenePlugin;

impl Plugin for PlayerMovementScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(
                OnEnter(Scene::PlayerMovement),
                (
                    player::spawn(Vec2::new(0.0, 0.0), 0.0),
                    background::spawn,
                    camera::spawn,
                    spawn,
                ),
            )
            .add_systems(
                OnExit(Scene::PlayerMovement),
                (
                    player::despawn,
                    background::despawn,
                    camera::despawn,
                    arena::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let arena = arena::Arena::new(1000.0, 200.0);
    arena.spawn_random_asteroids(&mut commands, &asset_server, 50);
}
