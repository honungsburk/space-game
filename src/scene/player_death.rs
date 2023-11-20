use bevy::prelude::*;

use super::Scene;
use crate::game::{background, camera, player, turret};

pub struct PlayerDeathScenePlugin;

impl Plugin for PlayerDeathScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(
                OnEnter(Scene::PlayerDeath),
                (
                    player::spawn(Vec2::new(0.0, 0.0), 0.0),
                    background::spawn,
                    camera::spawn,
                    spawn,
                ),
            )
            .add_systems(
                OnExit(Scene::PlayerDeath),
                (
                    player::despawn,
                    background::despawn,
                    camera::despawn,
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
    )
}
