mod gamepad;
pub mod player;
mod projectile;
mod systems;
mod weapon;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use player::PlayerPlugin;
use systems::*;
use weapon::WeaponPlugin;

use crate::events::GameOver;
use gamepad::GamepadPlugin;
use projectile::ProjectilePlugin;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        // Events
        .add_event::<GameOver>()
        // States
        .add_state::<SimulationState>()
        // Systems
        .add_plugins((PlayerPlugin, GamepadPlugin, ProjectilePlugin, WeaponPlugin))
        .add_systems(
            Update,
            (pause_simulation, toggle_simulation, resume_simulation),
        );
    }
}

// Simulation State

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
