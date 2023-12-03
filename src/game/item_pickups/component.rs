use super::Pickup;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Hash, Component)]
pub struct PickupLabel {
    pub kind: Pickup,
}

impl PickupLabel {
    pub fn new(kind: Pickup) -> Self {
        Self { kind }
    }

    pub fn experience(amount: u64) -> Self {
        Self::new(Pickup::Experience(amount))
    }
}

#[derive(Debug, Clone, PartialEq, Component)]
pub struct PickupRadius {
    radius: f32,
}

impl PickupRadius {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn can_pickup(&self, distance: f32) -> bool {
        distance <= self.radius
    }
}
