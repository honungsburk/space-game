use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
// Actions

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    ThrottleForward,
    ThrottleBackwards,
    RotateShip,
    RotateShipLeft,
    RotateShipRight,
    FireWeapon,
}
