pub mod actions;
pub mod components;
mod systems;

use actions::*;
use leafwing_input_manager::prelude::*;
use systems::*;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            // Configure System Sets
            // .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    control_ship,
                    fire_weapon,
                    update_player_rotation,
                    player_collision,
                ),
            );

        // Systems
        // On Exit State
        // .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
