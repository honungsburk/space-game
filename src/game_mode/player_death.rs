use bevy::prelude::*;

use super::GameMode;
use crate::game::{assets::AssetDB, background, camera, player, turret};

pub struct PlayerDeathPlugin;

impl Plugin for PlayerDeathPlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(
                OnEnter(GameMode::PlayerDeath),
                (
                    player::spawn(Vec2::new(0.0, 0.0), 0.0),
                    background::spawn,
                    camera::spawn,
                    spawn,
                ),
            )
            .add_systems(
                OnExit(GameMode::PlayerDeath),
                (
                    player::despawn,
                    background::despawn,
                    camera::despawn,
                    turret::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_db: Res<AssetDB>, asset_server: Res<AssetServer>) {
    let turret_location = Vec2::new(200.0, 0.0);
    let spawn_transform = Transform::from_translation(turret_location.extend(0.0));

    turret::spawn(
        &mut commands,
        &asset_db,
        &asset_server,
        &turret::TurretConfig::new(100, 1000),
        spawn_transform,
    )
}
