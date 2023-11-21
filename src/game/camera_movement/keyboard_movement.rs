use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::game::input::InputAction;

use super::CameraMovementAction;

/// Control Camera with WASD or arrow keys
///
/// - `speed`: Movement speed in pixels per second
///
#[derive(Component, Debug, Clone, PartialEq)]
pub struct KeyboardMovement {
    speed: f32,
}

impl Default for KeyboardMovement {
    fn default() -> Self {
        Self { speed: 100.0 }
    }
}

impl KeyboardMovement {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

pub fn update(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &KeyboardMovement)>,
    input_query: Query<&ActionState<InputAction>, Without<KeyboardMovement>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let input = input_query.single();

        for action in input.get_pressed() {
            if let InputAction::CameraMovement(camera_action) = action {
                println!("Camera action: {:?}", camera_action);
                match camera_action {
                    CameraMovementAction::MoveUp => direction.y += 1.0,
                    CameraMovementAction::MoveDown => direction.y -= 1.0,
                    CameraMovementAction::MoveLeft => direction.x -= 1.0,
                    CameraMovementAction::MoveRight => direction.x += 1.0,
                    _ => {}
                }
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * movement.speed * time.delta_seconds();
    }
}
