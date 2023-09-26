mod player;
mod systems;

use bevy::prelude::*;
use player::PlayerPlugin;
use systems::*;

use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // Systems
            .add_plugins(PlayerPlugin)
            .add_systems(
                Update,
                (pause_simulation, toggle_simulation, resume_simulation),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
