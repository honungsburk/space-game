use bevy::prelude::*;

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

#[derive(Component)]
pub struct Health(pub u32);

impl Health {
    pub fn new(health: u32) -> Self {
        Self(health)
    }

    pub fn health(&self) -> u32 {
        return self.0;
    }

    pub fn take_damage(&mut self, damage: &Damage) {
        self.0 -= damage.0;
    }

    pub fn is_alive(&self) -> bool {
        return self.0 > 0;
    }

    pub fn is_dead(&self) -> bool {
        return self.0 <= 0;
    }
}

// Time To Live

#[derive(Component)]
pub struct TimeToLive(pub Timer);

impl TimeToLive {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
    pub fn from_seconds(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}
