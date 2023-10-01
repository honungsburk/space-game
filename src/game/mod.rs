pub mod assets;
mod camera;
mod gamepad;
mod meteors;
pub mod player;
mod projectile;
mod systems;
mod weapon;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use meteors::MeteorPlugin;
use player::PlayerPlugin;
use systems::*;
use weapon::WeaponPlugin;

use crate::events::GameOver;
use assets::AssetPlugin;
use camera::CameraPlugin;
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
            CameraPlugin,
            AssetPlugin,
            PlayerPlugin,
            GamepadPlugin,
            ProjectilePlugin,
            WeaponPlugin,
            MeteorPlugin,
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
