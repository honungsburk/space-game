use bevy::prelude::*;

// Game Over event
#[derive(Event, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct GameOverEvent {
    pub score: u64,
}

// Game Over event
#[derive(Event, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct HighScoreEvent {
    pub player_name: String,
    pub placement: usize,
    pub score: u64,
}
