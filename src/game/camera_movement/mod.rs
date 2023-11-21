//!
//! What is a camera?
//!
//! Well, in a game it is something that has a location, zoom, and rotation.
//! And it has a viewport, which is the area of the screen that it is rendering to.
//!
//! Then there is camera movement: how does the camera move?
//! - The camera moves with wasd.
//! - The camera moves with the mouse.
//! - The camera follows the an entity.
//!
//! How to make composable camera movement?
//!
//!
//! I want to be able to do this:
//!
//! ```rust
//!
//! const camera = command.spawn(Camera2dBundle::default()).insert((
//!     CameraMovement::FollowEntity(ENTITY_ID, CameraMovement::Velcoity),
//!     CameraMovement::ScreenShake(ENTITY_ID),
//!     CameraMovement::MouseMovement(1.0),
//!     CameraMovement::WASDMovement(1.0),
//!     CameraMovement::ScrollToZoom(1.0),
//!     CameraMovement::MouseRotate(1.0),
//! ))
//! )
//!
//! ```
//!
mod keyboard_movement;

use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {}
}
