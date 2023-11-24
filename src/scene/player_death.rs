use bevy::prelude::*;

use super::GameScene;
use crate::game::{background, player, player_camera, turret};

pub struct PlayerDeathScenePlugin;

impl Plugin for PlayerDeathScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(OnEnter(GameScene::PlayerDeath), (background::spawn, spawn))
            .add_systems(
                OnExit(GameScene::PlayerDeath),
                (
                    player::despawn,
                    background::despawn,
                    player_camera::despawn,
                    turret::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let turret_location = Vec2::new(200.0, 0.0);
    let spawn_transform = Transform::from_translation(turret_location.extend(0.0));

    turret::spawn(
        &mut commands,
        &asset_server,
        &turret::TurretConfig::new(100, 1000),
        spawn_transform,
    );

    let player_entity =
        player::spawn_player(&mut commands, &asset_server, Vec2::new(0.0, 0.0), 0.0);

    player_camera::spawn(&mut commands, player_entity);
}
