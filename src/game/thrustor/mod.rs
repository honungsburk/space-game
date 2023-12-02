mod angular_thrustor;
mod linear_thrustor;

pub use angular_thrustor::AngularThrustor;
pub use linear_thrustor::LinearThrustor;

use bevy::prelude::*;

/// Represents a plugin for the thrustor system.
pub struct ThrustorPlugin;

impl Plugin for ThrustorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (angular_thrustor::update, linear_thrustor::update));
    }
}
