use bevy::prelude::*;

use super::Scene;
use crate::game::{arena, background, camera};

pub struct MainGameScenePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for MainGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Scene::MainGame),
            (background::spawn, camera::spawn, arena::spawn),
        )
        .add_systems(
            OnExit(Scene::MainGame),
            (background::despawn, camera::despawn, arena::despawn),
        );
    }
}
