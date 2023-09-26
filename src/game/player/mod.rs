mod components;
mod systems;

use systems::*;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            // .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_systems(Startup, spawn_player);
        // Systems
        // On Exit State
        // .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
