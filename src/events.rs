use bevy::prelude::*;

// When the game is exited
#[derive(Event, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct ExitGame();
// We want to save all data. use checksums to detext failures maybe?
// emit https://docs.rs/bevy/latest/bevy/app/struct.AppExit.html
// https://github.com/Zeenobit/moonshine_save
