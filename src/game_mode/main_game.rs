use bevy::prelude::*;

use super::GameMode;
use crate::game::{
    arena,
    assets::AssetDB,
    camera,
    player::{self, components::Player},
};

pub struct MainGamePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(arena::Arena::new(2000.0, 400.0))
            .insert_resource(arena::EnemySpawnTimer::from_seconds(10.0))
            .add_systems(
                OnEnter(GameMode::MainGame),
                (setup_main_game, camera::spawn, arena::spawn_arena),
            )
            .add_systems(
                OnExit(GameMode::MainGame),
                (despawn_player, camera::despawn, arena::despawn_arena),
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

fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    player::despawn_player(&mut commands, &player_query)
}
