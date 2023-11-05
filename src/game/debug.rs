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
use super::input::InputAction;
use bevy::prelude::*;
use bevy_rapier2d::prelude::DebugRenderContext;
use leafwing_input_manager::prelude::ActionState;

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
/// Actions
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugAction {
    BackgroundGrid,
    CameraPosition,
    CameraSetpoint,
    Render,
    FPSCounter,
}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

fn debug_keyboard_input(
    input_query: Query<&ActionState<InputAction>>,
    mut background_debug: Option<ResMut<background::BackgroundGridDebugFlag>>,
    mut camera_position_debug: Option<ResMut<camera::CameraPositionDebugFlag>>,
    mut camera_setpoint_debug: Option<ResMut<camera::CameraSetpointDebugFlag>>,
    mut render_debug: Option<ResMut<DebugRenderContext>>,
    mut fps_debug: Option<ResMut<hud::FPSCounterDebugFlag>>,
) {
    if let Ok(input_action) = input_query.get_single() {
        if input_action.just_pressed(InputAction::DebugBackgroundGrid) {
            if let Some(background_debug) = background_debug.as_mut() {
                background_debug.flag.flip();
            }
        }

        if input_action.just_pressed(InputAction::DebugCameraPosition) {
            if let Some(camera_position_debug) = camera_position_debug.as_mut() {
                camera_position_debug.flag.flip();
            }
        }

        if input_action.just_pressed(InputAction::DebugCameraSetpoint) {
            if let Some(camera_setpoint_debug) = camera_setpoint_debug.as_mut() {
                camera_setpoint_debug.flag.flip();
            }
        }

        if input_action.just_pressed(InputAction::DebugRender) {
            if let Some(render_debug) = render_debug.as_mut() {
                render_debug.enabled = !render_debug.enabled;
            }
        }

        if input_action.just_pressed(InputAction::DebugFPSCounter) {
            if let Some(fps_debug) = fps_debug.as_mut() {
                fps_debug.flag.flip();
            }
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
