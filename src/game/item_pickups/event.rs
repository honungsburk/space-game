use bevy::prelude::*;

use super::Pickup;

/// An event that is fired when a pickup is picked up.
#[derive(Debug, Clone, PartialEq, Hash, Event)]
pub struct PickupEvent(pub Pickup);
