use bevy::prelude::*;

use super::GameScene;
use crate::game::{arena, background};

pub struct MainGameScenePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for MainGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::MainGame),
            (background::spawn, arena::spawn),
        )
        .add_systems(
            OnExit(GameScene::MainGame),
            (background::despawn, arena::despawn),
        );
    }
}
