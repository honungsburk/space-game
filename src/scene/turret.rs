use bevy::prelude::*;

use super::Scene;
use crate::game::{background, camera, player, turret};

pub struct TurretScenePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for TurretScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(
                OnEnter(Scene::Turret),
                (
                    player::spawn(Vec2::new(0.0, 0.0), 0.0),
                    background::spawn,
                    camera::spawn,
                    spawn,
                ),
            )
            .add_systems(
                OnExit(Scene::Turret),
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
    let turret_location = Vec3::new(0.0, 600.0, 0.0);
    let spawn_transform = Transform::from_translation(turret_location);

    turret::spawn(
        &mut commands,
        &asset_server,
        &turret::TurretConfig::default(),
        spawn_transform,
    );
}
