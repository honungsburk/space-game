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
//!     CameraMovement::KeyboardMovement(200.0),
//!     CameraMovement::ScrollToZoom(1.0),
//!     CameraMovement::MouseRotate(1.0),
//! ))
//! )
//!
//! ```
//!
mod follow_entity_movement;
mod keyboard_movement;
mod shaky_movement;

use bevy::prelude::*;

pub use follow_entity_movement::{FollowEntityMovement, FollowEntityMovementBundle};
pub use keyboard_movement::{KeyboardMovement, KeyboardMovementBundle};
pub use shaky_movement::ShakyMovement;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            keyboard_movement::KeyboardMovementPlugin,
            follow_entity_movement::FollowEntityMovementPlugin,
            shaky_movement::ShakyMovementPlugin,
        ));
    }
}
