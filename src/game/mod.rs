mod arena;
pub mod assets;
pub mod average_velocity;
mod camera;
pub mod components;
mod enemy;
mod gamepad;
mod meteors;
pub mod player;
mod projectile;
mod systems;
pub mod trauma;
pub mod turret;
mod weapon;

use bevy::prelude::*;
use player::PlayerPlugin;
use systems::*;
use weapon::WeaponPlugin;

use crate::events::GameOver;
use arena::ArenaPlugin;
use assets::AssetPlugin;
use camera::CameraPlugin;
// use enemy::EnemyPlugin;
// use gamepad::GamepadPlugin;
use projectile::ProjectilePlugin;

use self::{average_velocity::AverageVelocityPlugin, trauma::TraumaPlugin, turret::TurretPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // Systems
            .add_plugins((
                // EnemyPlugin,
                ArenaPlugin,
                CameraPlugin,
                AssetPlugin,
                PlayerPlugin,
                TurretPlugin,
                // GamepadPlugin,
                ProjectilePlugin,
                WeaponPlugin,
                TraumaPlugin,
                AverageVelocityPlugin,
            ))
            .add_systems(
                Update,
                (
                    pause_simulation,
                    toggle_simulation,
                    resume_simulation,
                    despawn_dead,
                ),
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
