//! Debugging tools
//!
//! Gathers all the debugging tools used in the game into one place.
//! Easily enable/disable them in game by hitting `CMD+D` on MacOS.
//!
//! TODO: CMD+D should open a debug menu, where you can enable/disable
//!       the different debug tools. These should be saved to a config and loaded
//!       on startup..

use super::config::Flag;
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
        app.add_systems(Update, debug_keyboard_input)
            .insert_resource(Flag::<BackgroundGridDebugFlagLabel>::new(
                "Background Grid",
                "Display the background grid",
                false,
            ))
            .insert_resource(Flag::<CameraPositionDebugFlagLabel>::new(
                "Camera Position",
                "Display the camera position",
                false,
            ))
            .insert_resource(Flag::<CameraSetpointDebugFlagLabel>::new(
                "Camera Setpoint",
                "Display the camera setpoint",
                false,
            ))
            .insert_resource(Flag::<FPSDebugFlagLabel>::new(
                "FPS Counter",
                "Display the FPS counter",
                false,
            ))
            .insert_resource(Flag::<VisionConeFlagLabel>::new(
                "Vision Cone",
                "Display the vision cone",
                false,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Actions
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum DebugAction {
    #[default]
    NoOp,
    BackgroundGrid,
    CameraPosition,
    CameraSetpoint,
    Render,
    FPSCounter,
    VisionCone,
}

////////////////////////////////////////////////////////////////////////////////
/// Flags
////////////////////////////////////////////////////////////////////////////////

/// A flag that can be used to enable/disable showing the background grid.
#[derive(Debug)]
pub struct BackgroundGridDebugFlagLabel;
pub type BackgroundGridDebugFlag = Flag<BackgroundGridDebugFlagLabel>;

/// A flag that can be used to enable/disable showing the camera position.
#[derive(Debug)]
pub struct CameraPositionDebugFlagLabel;
pub type CameraPositionDebugFlag = Flag<CameraPositionDebugFlagLabel>;

/// A flag that can be used to enable/disable showing the camera setpoint.
#[derive(Debug)]
pub struct CameraSetpointDebugFlagLabel;
pub type CameraSetpointDebugFlag = Flag<CameraSetpointDebugFlagLabel>;

/// A flag that can be used to enable/disable showing the FPS counter.
#[derive(Debug)]
pub struct FPSDebugFlagLabel;
pub type FPSDebugFlag = Flag<FPSDebugFlagLabel>;

/// A flag that can be used to enable/disable showing the vision cone.
#[derive(Debug)]
pub struct VisionConeFlagLabel;
pub type VisionConeDebugFlag = Flag<VisionConeFlagLabel>;

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

/// Check if a flag is on
pub fn flag_is_on<A: Send + Sync + 'static>(flag: Option<Res<Flag<A>>>) -> bool {
    flag.map(|flag| flag.is_on()).unwrap_or(false)
}

fn debug_keyboard_input(
    input_query: Query<&ActionState<InputAction>>,
    mut background_debug: Option<ResMut<BackgroundGridDebugFlag>>,
    mut camera_position_debug: Option<ResMut<CameraPositionDebugFlag>>,
    mut camera_setpoint_debug: Option<ResMut<CameraSetpointDebugFlag>>,
    mut render_debug: Option<ResMut<DebugRenderContext>>,
    mut fps_debug: Option<ResMut<FPSDebugFlag>>,
    mut vision_cone_debug: Option<ResMut<VisionConeDebugFlag>>,
) {
    if let Ok(input_action) = input_query.get_single() {
        for action in input_action.get_just_pressed() {
            if let InputAction::Debug(debug_action) = action {
                match debug_action {
                    DebugAction::BackgroundGrid => {
                        background_debug.as_mut().map(|flag| flag.flip());
                    }
                    DebugAction::CameraPosition => {
                        camera_position_debug.as_mut().map(|flag| flag.flip());
                    }
                    DebugAction::CameraSetpoint => {
                        camera_setpoint_debug.as_mut().map(|flag| flag.flip());
                    }
                    DebugAction::Render => {
                        render_debug.as_mut().map(|ctx| ctx.enabled = !ctx.enabled);
                    }
                    DebugAction::FPSCounter => {
                        fps_debug.as_mut().map(|flag| flag.flip());
                    }
                    DebugAction::VisionCone => {
                        vision_cone_debug.as_mut().map(|flag| flag.flip());
                    }
                    _ => {}
                }
            }
        }
    }
}
