use bevy::prelude::*;

/// Enum for all the different types of game entities
///
/// Usefull to decouple game systems from one another.
///
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEntityType {
    Player,
    Enemy,
    Other,
}

// Enemy tag
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;
