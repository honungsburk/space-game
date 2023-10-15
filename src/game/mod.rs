mod arena;
pub mod assets;
pub mod average_velocity;
mod camera;
mod enemy;
pub mod game_entity;
mod gamepad;
mod meteors;
pub mod player;
mod projectile;
pub mod score;
mod systems;
pub mod time_to_live;
pub mod trauma;
pub mod turret;
pub mod vitality;
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
use projectile::ProjectilePlugin;

use self::{
    average_velocity::AverageVelocityPlugin, score::ScorePlugin, time_to_live::TimeToLivePlugin,
    trauma::TraumaPlugin, turret::TurretPlugin, vitality::VitalityPlugin,
};

pub struct GamePlugin {
    pub has_camera_debug: bool,
    pub has_colliders_debug: bool,
}

impl Default for GamePlugin {
    fn default() -> Self {
        Self {
            has_camera_debug: false,
            has_colliders_debug: false,
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Events
        .add_event::<GameOver>()
        // States
        .add_state::<SimulationState>()
        // Systems
        .add_plugins((
            // EnemyPlugin,
            ArenaPlugin,
            CameraPlugin::new(self.has_camera_debug),
            AssetPlugin,
            PlayerPlugin,
            TurretPlugin,
            // GamepadPlugin,
            ProjectilePlugin,
            WeaponPlugin,
            TraumaPlugin,
            AverageVelocityPlugin,
            TimeToLivePlugin,
            VitalityPlugin,
            ScorePlugin,
        ))
        .add_systems(
            Update,
            (pause_simulation, toggle_simulation, resume_simulation),
        );

        if self.has_colliders_debug {
            app.add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}

// Simulation State

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
