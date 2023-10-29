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
    mut background_debug: ResMut<background::BackgroundDebug>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        background_debug.toggle_grid_lines();
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Re-exports
////////////////////////////////////////////////////////////////////////////////

pub mod config {
    pub use crate::game::background::BackgroundDebug;
}
