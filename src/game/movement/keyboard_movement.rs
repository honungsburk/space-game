use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct KeyboardMovementPlugin;

impl Plugin for KeyboardMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<KeyboardMovementAction>::default())
            .add_systems(Update, update);
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Bundle
////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct KeyboardMovementBundle {
    pub keyboard_movement: KeyboardMovement,
    pub input: InputManagerBundle<KeyboardMovementAction>,
}

impl Default for KeyboardMovementBundle {
    fn default() -> Self {
        Self {
            keyboard_movement: KeyboardMovement::default(),
            input: InputManagerBundle {
                action_state: ActionState::default(),
                input_map: create_input_map(),
            },
        }
    }
}

impl KeyboardMovementBundle {
    pub fn new(speed: f32) -> Self {
        Self {
            keyboard_movement: KeyboardMovement::new(speed),
            ..default()
        }
    }
}

fn create_input_map() -> InputMap<KeyboardMovementAction> {
    let mut input_map: InputMap<KeyboardMovementAction> = InputMap::default();

    // Add Camera inputs
    input_map.insert_multiple(vec![
        (
            InputKind::Keyboard(KeyCode::Up),
            KeyboardMovementAction::MoveUp,
        ),
        (
            InputKind::Keyboard(KeyCode::Down),
            KeyboardMovementAction::MoveDown,
        ),
        (
            InputKind::Keyboard(KeyCode::Left),
            KeyboardMovementAction::MoveLeft,
        ),
        (
            InputKind::Keyboard(KeyCode::Right),
            KeyboardMovementAction::MoveRight,
        ),
        (
            InputKind::Keyboard(KeyCode::W),
            KeyboardMovementAction::MoveUp,
        ),
        (
            InputKind::Keyboard(KeyCode::S),
            KeyboardMovementAction::MoveDown,
        ),
        (
            InputKind::Keyboard(KeyCode::A),
            KeyboardMovementAction::MoveLeft,
        ),
        (
            InputKind::Keyboard(KeyCode::D),
            KeyboardMovementAction::MoveRight,
        ),
    ]);

    input_map
}

////////////////////////////////////////////////////////////////////////////////
/// Components
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
/// Ations
////////////////////////////////////////////////////////////////////////////////

/// Actions that move the camera
#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum KeyboardMovementAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn update(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &KeyboardMovement,
        &ActionState<KeyboardMovementAction>,
    )>,
) {
    for (mut transform, movement, input) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        for action in input.get_pressed() {
            match action {
                KeyboardMovementAction::MoveUp => direction.y += 1.0,
                KeyboardMovementAction::MoveDown => direction.y -= 1.0,
                KeyboardMovementAction::MoveLeft => direction.x -= 1.0,
                KeyboardMovementAction::MoveRight => direction.x += 1.0,
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * movement.speed * time.delta_seconds();
    }
}
