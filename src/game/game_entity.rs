use bevy::prelude::*;

/// Enum for all the different types of game entities
///
/// Usefull to decouple game systems from one another.
///
/// TODO: This is a stupid type. Can be replaced with Or<(With<Player>, With<Enemy>)> which is more general.
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
