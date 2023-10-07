mod arena;
pub mod assets;
mod camera;
mod enemy;
mod gamepad;
mod meteors;
pub mod player;
mod projectile;
mod systems;
mod weapon;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use systems::*;
use weapon::WeaponPlugin;

use crate::events::GameOver;
use arena::ArenaPlugin;
use assets::AssetPlugin;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
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
        .add_plugins((
            EnemyPlugin,
            ArenaPlugin,
            CameraPlugin,
            AssetPlugin,
            PlayerPlugin,
            GamepadPlugin,
            ProjectilePlugin,
            WeaponPlugin,
        ))
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
