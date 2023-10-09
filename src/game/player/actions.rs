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

impl PlayerAction {
    pub fn is_throttle_forward(&self) -> bool {
        match self {
            PlayerAction::ThrottleForward => true,
            _ => false,
        }
    }

    pub fn is_throttle(&self) -> bool {
        match self {
            PlayerAction::ThrottleForward => true,
            PlayerAction::ThrottleBackwards => true,
            _ => false,
        }
    }

    pub fn is_rotate(&self) -> bool {
        match self {
            PlayerAction::RotateShip => true,
            PlayerAction::RotateShipLeft => true,
            PlayerAction::RotateShipRight => true,
            _ => false,
        }
    }

    pub fn is_fire(&self) -> bool {
        match self {
            PlayerAction::FireWeapon => true,
            _ => false,
        }
    }
}
