use super::player::actions::PlayerAction;
use bevy::input::gamepad::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (gamepad_connections, on_change_gamepad, gamepad_system),
        );
    }
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}

/// Simple resource to store the ID of the
/// connected gamepad. We need to know which
/// gamepad to use for player input.
#[derive(Debug, Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for gamepad_event in gamepad_evr.iter() {
        match gamepad_event {
            GamepadEvent::Connection(connection_event) => {
                match &connection_event.connection {
                    GamepadConnection::Connected(gamepad_info) => {
                        // dbg!(info);
                        println!(
                            "{:?} - {:?}: New gamepad connected!",
                            connection_event.gamepad.id, gamepad_info.name
                        );

                        // if we don't have any gamepad yet, use
                        // this one
                        if my_gamepad.is_none() {
                            commands.insert_resource(MyGamepad(connection_event.gamepad));
                        }
                    }
                    GamepadConnection::Disconnected { .. } => {
                        println!("{:?}: Gamepad disconnected!", connection_event.gamepad.id);

                        // if it's the one we previously associated
                        // with the player,
                        // disassociate it:
                        if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                            if *old_id == connection_event.gamepad {
                                commands.remove_resource::<MyGamepad>();
                            }
                        }
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn on_change_gamepad(
    gamepad: Option<Res<MyGamepad>>,
    mut input_map: Query<&mut InputMap<PlayerAction>>,
) {
    if let Some(gamepad) = gamepad {
        if gamepad.is_changed() {
            for mut map in input_map.iter_mut() {
                map.set_gamepad(gamepad.0);
            }
        }
    }
}
