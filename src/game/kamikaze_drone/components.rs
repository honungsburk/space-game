use bevy::prelude::{Component, Entity};

use std::collections::HashSet;

/// Means an entity is a KamikazeDrone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct KamikazeDroneLabel;

/// Means an entity is a KamikazeDroneSensorLabel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct KamikazeDroneSensorLabel;

#[derive(Debug, PartialEq, Eq, Component)]
pub struct BoidTargets(pub HashSet<Entity>);
