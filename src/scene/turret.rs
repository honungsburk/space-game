use bevy::prelude::*;

use super::GameScene;
use crate::game::{background, meteors, player, player_camera, turret};

pub struct TurretScenePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for TurretScenePlugin {
    fn build(&self, app: &mut App) {
        app // Runs even when the game is paused
            .add_systems(OnEnter(GameScene::Turret), (background::spawn, spawn))
            .add_systems(
                OnExit(GameScene::Turret),
                (
                    player::despawn,
                    background::despawn,
                    player_camera::despawn,
                    turret::despawn,
                    crate::utility_systems::cleanup::<meteors::Meteor>,
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

    meteors::spawn_immovable_meteor(
        &asset_server,
        &mut commands,
        meteors::MeteorSize::Big,
        Transform::from_translation(Vec3::new(200.0, 600.0, 0.0)),
    );

    let player_entity =
        player::spawn_player(&mut commands, &asset_server, Vec2::new(0.0, 0.0), 0.0);

    player_camera::spawn(&mut commands, player_entity);
}
