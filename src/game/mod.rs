pub mod arena;
pub mod assets;
pub mod average_velocity;
pub mod background;
pub mod boids;
pub mod camera;
pub mod camera_movement;
pub mod config;
pub mod control_system;
pub mod debug;
pub mod enemy;
pub mod events;
pub mod game_entity;
pub mod kamikaze_drone;
pub mod meteors;
pub mod player;
pub mod projectile;
pub mod score;
pub mod sensor;
mod systems;
pub mod time_to_live;
pub mod trauma;
pub mod turret;
pub mod vitality;
pub mod weapon;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use systems::*;
use weapon::WeaponPlugin;

use arena::ArenaPlugin;
use camera::CameraPlugin;
use projectile::ProjectilePlugin;

use self::{
    average_velocity::AverageVelocityPlugin, background::BackgroundPlugin, boids::BoidsPlugin,
    camera_movement::CameraMovementPlugin, control_system::ControlSystemPlugin, debug::DebugPlugin,
    enemy::EnemyPlugin, events::GameOverEvent, kamikaze_drone::KamikazeDronesPlugin,
    score::ScorePlugin, sensor::SensorPlugin, time_to_live::TimeToLivePlugin, trauma::TraumaPlugin,
    turret::TurretPlugin, vitality::VitalityPlugin,
};

pub struct GamePlugin;

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
            SensorPlugin,
            BoidsPlugin,
            KamikazeDronesPlugin,
            EnemyPlugin,
            ControlSystemPlugin,
            CameraMovementPlugin,
        ))
        .add_plugins((
            DebugPlugin,
            BackgroundPlugin,
            ArenaPlugin,
            CameraPlugin,
            PlayerPlugin,
            TurretPlugin,
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
