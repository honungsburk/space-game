use bevy::prelude::Component;

/// Means an entity is a KamikazeDrone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BoidLabel;
