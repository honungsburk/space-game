use bevy::prelude::*;

use super::GameMode;
use crate::game::{arena, background, camera};

pub struct MainGamePlugin;

// TODO: Move the Arena code to the main_game.rs file

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMode::MainGame),
            (background::spawn, camera::spawn, arena::spawn),
        )
        .add_systems(
            OnExit(GameMode::MainGame),
            (background::despawn, camera::despawn, arena::despawn),
        );
    }
}
