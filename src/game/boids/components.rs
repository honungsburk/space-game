use bevy::prelude::{Component, Entity};

use std::collections::HashSet;

/// Means an entity is a KamikazeDrone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BoidLabel;
