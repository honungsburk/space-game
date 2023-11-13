use bevy::prelude::Component;

/// Means an entity is a KamikazeDrone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct KamikazeDroneLabel;

/// Means an entity is a KamikazeDroneSensorLabel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct KamikazeDroneSensorLabel;
