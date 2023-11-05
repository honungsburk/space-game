pub mod health;

use bevy::prelude::*;

pub use health::*;

/// Set enum for the systems relating to vitality
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum VitalitySystem {
    /// Check if any entities are dead and despawn them
    DeathCheck,
}

use super::game_entity::GameEntityType;

pub struct VitalityPlugin;

impl Plugin for VitalityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
            .add_systems(Update, update_death.in_set(VitalitySystem::DeathCheck));
    }
}

// Damage

#[derive(Component)]
pub struct Damage(pub u32);

impl Damage {
    pub fn new(damage: u32) -> Self {
        Self(damage)
    }

    pub fn damage(&self) -> u32 {
        return self.0;
    }
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeathEvent {
    entity: Entity,
    _type: GameEntityType,
}

impl DeathEvent {
    pub fn new(entity: Entity, _type: GameEntityType) -> Self {
        Self { entity, _type }
    }

    pub fn entity(&self) -> Entity {
        return self.entity;
    }

    pub fn _type(&self) -> GameEntityType {
        return self._type;
    }
}

pub fn update_death(
    mut commands: Commands,
    mut death_event_writer: EventWriter<DeathEvent>,
    query: Query<(Entity, &Health, Option<&GameEntityType>)>,
) {
    for (entity, health, game_entity_type) in query.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();

            if let Some(game_entity_type) = game_entity_type {
                death_event_writer.send(DeathEvent::new(entity, *game_entity_type));
            }
        }
    }
}
