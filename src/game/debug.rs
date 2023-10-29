//! Debugging tools
//!
//! Gathers all the debugging tools used in the game into one place.
//! Easily enable/disable them in game by hitting `CMD+D` on MacOS.
//!
//! TODO: CMD+D should open a debug menu, where you can enable/disable
//!       the different debug tools. These should be saved to a config and loaded
//!       on startup..

use bevy::prelude::*;

use super::background;
use super::camera;

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_keyboard_input);
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

fn debug_keyboard_input(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut background_debug: Option<ResMut<background::BackgroundDebug>>,
    mut camera_position_debug: Option<ResMut<camera::CameraPositionDebugFlag>>,
    mut camera_setpoint_debug: Option<ResMut<camera::CameraSetpointDebugFlag>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(background_debug) = background_debug.as_mut() {
            background_debug.toggle_grid_lines();
        }
    }

    if keyboard_input.just_pressed(KeyCode::F2) {
        if let Some(camera_position_debug) = camera_position_debug.as_mut() {
            camera_position_debug.flag.flip();
        }
    }

    if keyboard_input.just_pressed(KeyCode::F3) {
        if let Some(camera_setpoint_debug) = camera_setpoint_debug.as_mut() {
            camera_setpoint_debug.flag.flip();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Re-exports
////////////////////////////////////////////////////////////////////////////////

pub mod config {
    pub use crate::game::background::BackgroundDebug;
    pub use crate::game::camera::CameraPositionDebugFlag;
    pub use crate::game::camera::CameraSetpointDebugFlag;
}
