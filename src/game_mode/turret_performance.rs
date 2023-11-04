use bevy::prelude::*;

use super::GameMode;
use crate::game::{arena, assets::AssetDB, background, camera, player};

pub struct MainGamePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(arena::Arena::new(2000.0, 400.0))
            .insert_resource(arena::EnemySpawnTimer::from_seconds(10.0)) // Runs even when the game is paused
            .add_systems(
                OnEnter(GameMode::MainGame),
                (
                    setup_main_game,
                    background::spawn,
                    camera::spawn,
                    arena::spawn_arena,
                ),
            )
            .add_systems(
                OnExit(GameMode::MainGame),
                (
                    player::despawn,
                    background::despawn,
                    camera::despawn,
                    arena::despawn_arena,
                ),
            );
    }
}
//

fn setup_main_game(
    mut commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
    arena: Res<arena::Arena>,
) {
    arena.spawn_player(&mut commands, &asset_db, &asset_server)
}
