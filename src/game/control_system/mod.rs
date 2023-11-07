//! # Control system module.
//!
//! This module is attached to player and enemy entities. It is responsible for
//! controlling the entity's movement and rotation. It takes a desired velocity
//! and rotation and applies forces to the entity to achieve the desired velocity
//! and rotation.
//!
//! The control system is implemented as a PID controller, and is in control of the
//! accelertion and rotation thrusters. The thrustors can be enabled/disabled, and given
//! different characteristics (e.g. max force, max acceleration, max speed, etc). The thrusters
//! can be automatically tuned to the entity's mass and moment of inertia which makes
//! it very simple to add new entities and tune existing entities.
//!
//!

mod direction_control;
mod thrustor;

pub use direction_control::DirectionControl;

use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ControlSystemPlugin;

impl Plugin for ControlSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, direction_control::update_direction_control);
    }
}

// mut query: Query<(&mut ExternalImpulse, &Transform, &mut DirectionControl), With<Player>>,
