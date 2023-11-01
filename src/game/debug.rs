//! Debugging tools
//!
//! Gathers all the debugging tools used in the game into one place.
//! Easily enable/disable them in game by hitting `CMD+D` on MacOS.
//!
//! TODO: CMD+D should open a debug menu, where you can enable/disable
//!       the different debug tools. These should be saved to a config and loaded
//!       on startup..

use crate::ui::hud;

use super::background;
use super::camera;
use bevy::prelude::*;
use bevy_rapier2d::prelude::DebugRenderContext;

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
    mut background_debug: Option<ResMut<background::BackgroundGridDebugFlag>>,
    mut camera_position_debug: Option<ResMut<camera::CameraPositionDebugFlag>>,
    mut camera_setpoint_debug: Option<ResMut<camera::CameraSetpointDebugFlag>>,
    mut render_debug: Option<ResMut<DebugRenderContext>>,
    mut fps_debug: Option<ResMut<hud::FPSCounterDebugFlag>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(background_debug) = background_debug.as_mut() {
            background_debug.flag.flip();
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

    if keyboard_input.just_pressed(KeyCode::F4) {
        if let Some(render_debug) = render_debug.as_mut() {
            render_debug.enabled = !render_debug.enabled;
        }
    }

    if keyboard_input.just_pressed(KeyCode::F5) {
        if let Some(fps_debug) = fps_debug.as_mut() {
            fps_debug.flag.flip();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Re-exports
////////////////////////////////////////////////////////////////////////////////

pub mod config {
    pub use crate::game::background::BackgroundGridDebugFlag;
    pub use crate::game::camera::CameraPositionDebugFlag;
    pub use crate::game::camera::CameraSetpointDebugFlag;
}
