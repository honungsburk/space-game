pub mod arena;
pub mod assets;
pub mod average_velocity;
pub mod background;
pub mod boids;
pub mod config;
pub mod debug;
pub mod enemy;
pub mod events;
pub mod game_entity;
pub mod guard_point;
pub mod kamikaze_drone;
pub mod meteors;
pub mod movement;
pub mod player;
pub mod player_camera;
pub mod player_mini;
pub mod projectile;
pub mod score;
pub mod screen_bounds;
pub mod sensor;
pub mod simple_enemy;
pub mod spawner;
mod systems;
pub mod thrustor;
pub mod time_to_live;
pub mod trauma;
pub mod turret;
pub mod vitality;
pub mod weapon;

use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use systems::*;
use weapon::WeaponPlugin;

use arena::ArenaPlugin;
use projectile::ProjectilePlugin;

use self::{
    average_velocity::AverageVelocityPlugin,
    background::BackgroundPlugin,
    boids::BoidsPlugin,
    debug::{DebugPlugin, VisualDebug},
    enemy::EnemyPlugin,
    events::GameOverEvent,
    kamikaze_drone::KamikazeDronesPlugin,
    movement::MovementPlugin,
    score::ScorePlugin,
    screen_bounds::ScreenBoundsPlugin,
    sensor::SensorPlugin,
    simple_enemy::SimpleEnemyPlugin,
    spawner::SpawnerPlugin,
    thrustor::ThrustorPlugin,
    time_to_live::TimeToLivePlugin,
    trauma::TraumaPlugin,
    turret::TurretPlugin,
    vitality::VitalityPlugin,
};

pub struct GamePlugin {
    pub visual_debug: HashSet<VisualDebug>,
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
            SensorPlugin,
            BoidsPlugin,
            KamikazeDronesPlugin,
            EnemyPlugin,
            ThrustorPlugin,
            MovementPlugin,
            ScreenBoundsPlugin,
            SimpleEnemyPlugin,
            SpawnerPlugin,
        ))
        .add_plugins((
            DebugPlugin {
                visual_debug: self.visual_debug.clone(),
            },
            BackgroundPlugin,
            ArenaPlugin,
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

        let mut rapier_debug_plugin = RapierDebugRenderPlugin::default();

        if !self.visual_debug.contains(&VisualDebug::Colliders) {
            rapier_debug_plugin = rapier_debug_plugin.disabled();
        }

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
