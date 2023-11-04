use bevy::prelude::*;

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

// Health

#[derive(Component)]
pub struct Health {
    current: u32,
    max: u32,
}

impl Health {
    pub fn at_max(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn new(current: u32, max: u32) -> Self {
        Self {
            current: current.min(max),
            max,
        }
    }

    pub fn current(&self) -> u32 {
        return self.current;
    }

    pub fn take_damage(&mut self, damage: &Damage) {
        self.current -= damage.0;
    }

    pub fn take_damage_u32(&mut self, damage: u32) {
        self.current -= damage;
    }

    pub fn heal(&mut self, amount: u32) {
        self.current += amount;
        self.current = self.current.min(self.max);
    }

    pub fn max(&self) -> u32 {
        return self.max;
    }

    /// Heal the entity by a percentage of its max health
    /// the percentage is clamped between 0 and 1
    pub fn heal_pct(&mut self, pct: f32) {
        self.heal((self.max as f32 * pct.clamp(0.0, 1.0)) as u32);
    }

    pub fn is_alive(&self) -> bool {
        return self.current > 0;
    }

    pub fn is_dead(&self) -> bool {
        return self.current <= 0;
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
