use bevy::prelude::Component;

/// Means an entity is a KamikazeDrone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct KamikazeDroneLabel;

/// Anything with this label can be targeted by a KamikazeDrone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct KamikazeDroneTargetLabel;
