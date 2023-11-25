//! Debugging tools
//!
//! Gathers all the debugging tools used in the game into one place.
//! Easily enable/disable them in game by hitting `CMD+D` on MacOS.
//!
//! TODO: CMD+D should open a debug menu, where you can enable/disable
//!       the different debug tools. These should be saved to a config and loaded
//!       on startup..

use std::collections::HashSet;

use super::config::Flag;
use bevy::prelude::*;
use bevy_rapier2d::prelude::DebugRenderContext;
use clap::ValueEnum;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DebugPlugin {
    pub visual_debug: HashSet<VisualDebug>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<VisualDebug>::default())
            .add_systems(Startup, create_debug_input)
            .add_systems(Update, debug_keyboard_input)
            .insert_resource(Flag::<BackgroundGridDebugFlagLabel>::new(
                "Background Grid",
                "Display the background grid",
                self.visual_debug.contains(&VisualDebug::BackgroundGrid),
            ))
            .insert_resource(Flag::<CameraPositionDebugFlagLabel>::new(
                "Camera Position",
                "Display the camera position",
                self.visual_debug.contains(&VisualDebug::CameraPosition),
            ))
            .insert_resource(Flag::<CameraSetpointDebugFlagLabel>::new(
                "Camera Setpoint",
                "Display the camera setpoint",
                self.visual_debug.contains(&VisualDebug::CameraSetpoint),
            ))
            .insert_resource(Flag::<FPSDebugFlagLabel>::new(
                "FPS Counter",
                "Display the FPS counter",
                self.visual_debug.contains(&VisualDebug::FPSCounter),
            ))
            .insert_resource(Flag::<VisionConeFlagLabel>::new(
                "Vision Cone",
                "Display the vision cone",
                self.visual_debug.contains(&VisualDebug::VisionCone),
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Actions
////////////////////////////////////////////////////////////////////////////////

/// Enum for all the different visual debug tools.
#[derive(
    Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, ValueEnum, Deserialize, Serialize,
)]
pub enum VisualDebug {
    BackgroundGrid,
    CameraPosition,
    CameraSetpoint,
    Render,
    FPSCounter,
    VisionCone,
}

pub fn create_input_map() -> InputMap<VisualDebug> {
    // Create an `InputMap` to add default inputs to
    let mut input_map: InputMap<VisualDebug> = InputMap::default();

    // Add Debug inputs
    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::F1),
            VisualDebug::BackgroundGrid,
        ),
        (
            InputKind::Keyboard(KeyCode::F2),
            VisualDebug::CameraPosition,
        ),
        (
            InputKind::Keyboard(KeyCode::F3),
            VisualDebug::CameraSetpoint,
        ),
        (InputKind::Keyboard(KeyCode::F4), VisualDebug::Render),
        (InputKind::Keyboard(KeyCode::F5), VisualDebug::FPSCounter),
        (InputKind::Keyboard(KeyCode::F6), VisualDebug::VisionCone),
    ]);

    input_map.build()
}

fn create_debug_input(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<VisualDebug> {
        action_state: ActionState::default(),
        input_map: create_input_map(),
    });
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
    debug_action_query: Query<&ActionState<VisualDebug>>,
    mut background_debug: Option<ResMut<BackgroundGridDebugFlag>>,
    mut camera_position_debug: Option<ResMut<CameraPositionDebugFlag>>,
    mut camera_setpoint_debug: Option<ResMut<CameraSetpointDebugFlag>>,
    mut render_debug: Option<ResMut<DebugRenderContext>>,
    mut fps_debug: Option<ResMut<FPSDebugFlag>>,
    mut vision_cone_debug: Option<ResMut<VisionConeDebugFlag>>,
) {
    if let Ok(input_action) = debug_action_query.get_single() {
        for action in input_action.get_just_pressed() {
            match action {
                VisualDebug::BackgroundGrid => {
                    background_debug.as_mut().map(|flag| flag.flip());
                }
                VisualDebug::CameraPosition => {
                    camera_position_debug.as_mut().map(|flag| flag.flip());
                }
                VisualDebug::CameraSetpoint => {
                    camera_setpoint_debug.as_mut().map(|flag| flag.flip());
                }
                VisualDebug::Render => {
                    render_debug.as_mut().map(|ctx| ctx.enabled = !ctx.enabled);
                }
                VisualDebug::FPSCounter => {
                    fps_debug.as_mut().map(|flag| flag.flip());
                }
                VisualDebug::VisionCone => {
                    vision_cone_debug.as_mut().map(|flag| flag.flip());
                }
            }
        }
    }
}
