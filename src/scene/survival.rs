use bevy::prelude::*;

use super::GameScene;
use crate::game::{
    background,
    movement::FollowEntityMovement,
    player, simple_enemy,
    spawner::{self, Spawner},
};

pub struct SurvivalScenePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for SurvivalScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Survival), (background::spawn, spawn))
            .add_systems(
                OnExit(GameScene::Survival),
                (
                    background::despawn,
                    player::despawn,
                    simple_enemy::despawn,
                    spawner::despawn,
                ),
            );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_entity =
        player::spawn_player(&mut commands, &asset_server, Vec2::new(0.0, 0.0), 0.0);

    commands
        .spawn(Camera2dBundle::default())
        .insert(FollowEntityMovement::basic(player_entity));

    commands.spawn(Spawner::new());
}
