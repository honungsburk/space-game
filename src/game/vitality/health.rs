use super::Damage;
use bevy::prelude::*;

/// Health component for entities. This component represents the health of an entity, with a current
/// value and a maximum value. The current value represents the entity's current health, while the
/// maximum value represents the entity's maximum possible health.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Health {
    current: u32,
    max: u32,
}

impl Health {
    /// Creates a new `Vitality` component with the current and maximum health set to the same value.
    ///
    /// ## Arguments
    ///
    /// * `max` - The maximum health value for the `Vitality` component.
    ///
    /// ## Examples
    ///
    /// ```
    /// use space_game::game::vitality::Health;
    ///
    /// let health = Health::at_max(100);
    ///
    /// assert_eq!(health.current(), 100);
    /// assert_eq!(health.max(), 100);
    /// ```
    pub fn at_max(max: u32) -> Self {
        Self { current: max, max }
    }

    /// Create a new health component with the given maximum health.
    ///
    /// ## Arguments
    ///
    /// * `current` - The current health value for the component.
    /// * `max` - The maximum health value for the component.
    ///
    /// ## Example
    ///
    /// ```
    /// use space_game::game::vitality::Health;
    ///
    /// let health = Health::new(100, 200);
    ///
    /// assert_eq!(health.current(), 100);
    /// assert_eq!(health.max(), 200);
    /// ```
    /// Create a new health component. Note that the current health is clamped between 0 and max.
    pub fn new(current: u32, max: u32) -> Self {
        Self {
            current: current.min(max),
            max,
        }
    }

    /// Get the current health
    pub fn current(&self) -> u32 {
        return self.current;
    }

    /// Take damage from a Damage component
    ///
    /// # Arguments
    ///
    /// * `damage` - The amount of damage to take
    ///
    /// # Examples
    ///
    /// ```
    /// use space_game::game::vitality::{Damage, Health};
    ///
    /// let mut health = Health::at_max(100);
    /// health.take_damage(&Damage(50));
    ///
    /// assert_eq!(health.current(), 50);
    /// ```
    pub fn take_damage(&mut self, damage: &Damage) {
        self.take_damage_u32(damage.0)
    }

    /// Take damage from a u32
    ///
    /// # Arguments
    ///
    /// * `damage` - The amount of damage to take
    ///
    /// # Examples
    ///
    /// ```
    /// use space_game::game::vitality::{Damage, Health};
    ///
    /// let mut health = Health::at_max(100);
    /// health.take_damage_u32(50);
    ///
    /// assert_eq!(health.current(), 50);
    /// ```
    pub fn take_damage_u32(&mut self, damage: u32) {
        // And the min to prevent underflow
        self.current = self.current.saturating_sub(damage)
    }

    /// Heal the entity by a u32
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of health to heal
    ///
    /// # Examples
    ///
    /// ```
    /// use space_game::game::vitality::{Damage, Health};
    ///
    /// let mut health = Health::at_max(100);
    /// health.take_damage_u32(50);
    /// health.heal(40);
    /// assert_eq!(health.current(), 90);
    ///
    /// health.heal(40);
    /// assert_eq!(health.current(), 100);
    /// ```
    pub fn heal(&mut self, amount: u32) {
        self.current += amount;
        self.current = self.current.min(self.max);
    }

    /// Get the max health
    pub fn max(&self) -> u32 {
        return self.max;
    }

    /// Heal the entity by a percentage of its max health
    /// the percentage is clamped between 0 and 1.
    ///
    /// # Arguments
    ///
    /// * `pct` - The percent of max health to heal. Clamped between 0 and 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use space_game::game::vitality::{Damage, Health};
    ///
    /// let mut health = Health::at_max(100);
    /// health.take_damage_u32(50);
    /// health.heal_percent(0.4);
    /// assert_eq!(health.current(), 90);
    ///
    /// health.heal_percent(0.4);
    /// assert_eq!(health.current(), 100);
    /// ```
    pub fn heal_percent(&mut self, pct: f32) {
        self.heal((self.max as f32 * pct.clamp(0.0, 1.0)) as u32);
    }

    /// If the entity is alive
    ///
    /// # Examples
    ///
    /// ```
    /// use space_game::game::vitality::{Damage, Health};
    ///
    /// let mut health = Health::at_max(100);
    /// assert_eq!(health.is_alive(), true);
    /// health.take_damage_u32(120);
    /// assert_eq!(health.is_alive(), false);
    /// ```
    pub fn is_alive(&self) -> bool {
        return self.current > 0;
    }

    /// If the entity is dead.
    ///
    /// # Examples
    ///
    /// ```
    /// use space_game::game::vitality::{Damage, Health};
    ///
    /// let mut health = Health::at_max(100);
    /// assert_eq!(health.is_dead(), false);
    /// health.take_damage_u32(120);
    /// assert_eq!(health.is_dead(), true);
    /// ```
    pub fn is_dead(&self) -> bool {
        return !self.is_alive();
    }
}
