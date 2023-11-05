pub mod arena;
pub mod assets;
pub mod average_velocity;
pub mod background;
pub mod camera;
pub mod config;
pub mod debug;
mod enemy;
pub mod events;
pub mod game_entity;
pub mod input;
pub mod meteors;
pub mod player;
mod projectile;
pub mod score;
mod systems;
pub mod targets;
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

use arena::ArenaPlugin;
use assets::AssetPlugin;
use camera::CameraPlugin;
use projectile::ProjectilePlugin;

use self::{
    average_velocity::AverageVelocityPlugin, background::BackgroundPlugin, debug::DebugPlugin,
    events::GameOverEvent, input::InputPlugin, score::ScorePlugin, time_to_live::TimeToLivePlugin,
    trauma::TraumaPlugin, turret::TurretPlugin, vitality::VitalityPlugin,
};

pub struct GamePlugin {
    pub has_camera_debug: bool,
    pub has_colliders_debug: bool,
    pub high_scores: score::HighScores,
}

impl Default for GamePlugin {
    fn default() -> Self {
        Self {
            has_camera_debug: false,
            has_colliders_debug: false,
            high_scores: score::HighScores::default(),
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
        .add_event::<GameOverEvent>()
        // States
        .add_state::<SimulationState>()
        // Systems
        .add_plugins((
            // EnemyPlugin,
            InputPlugin,
            DebugPlugin,
            BackgroundPlugin,
            ArenaPlugin,
            CameraPlugin::new(self.has_camera_debug),
            AssetPlugin,
            PlayerPlugin,
            TurretPlugin,
            ProjectilePlugin,
            WeaponPlugin,
            TraumaPlugin,
            AverageVelocityPlugin,
            TimeToLivePlugin,
            VitalityPlugin,
            ScorePlugin {
                high_scores: self.high_scores.clone(),
            },
        ))
        .add_systems(
            Update,
            (pause_simulation, toggle_simulation, resume_simulation),
        );

        let rapier_debug_plugin = RapierDebugRenderPlugin::default().disabled();

        app.add_plugins(rapier_debug_plugin);
    }
}

// Simulation State

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
