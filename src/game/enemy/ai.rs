use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct ShipAI {
    pub state: ShipState,
    // target_group: Group, //Used to allow turrets to target different groups, player, enemies, etc
}

impl Default for ShipAI {
    fn default() -> Self {
        Self {
            state: ShipState::default(),
            // target_group: Group::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShipState {
    Idle,
    Targeting { timer: Timer },
    Fire,
}

impl Default for ShipState {
    fn default() -> Self {
        Self::Idle
    }
}

impl ShipState {
    pub fn is_firing(&self) -> bool {
        match self {
            Self::Fire => true,
            _ => false,
        }
    }

    pub fn is_targeting(&self) -> bool {
        match self {
            Self::Targeting { .. } => true,
            _ => false,
        }
    }
    pub fn is_idle(&self) -> bool {
        match self {
            Self::Idle => true,
            _ => false,
        }
    }

    pub fn update(&mut self, time: &Time, has_target: bool) {
        match self {
            Self::Idle => {
                if has_target {
                    *self = Self::Targeting {
                        timer: Timer::from_seconds(1.0, TimerMode::Once),
                    };
                }
            }
            Self::Targeting { timer } => {
                if has_target {
                    if timer.tick(time.delta()).just_finished() {
                        *self = Self::Fire;
                    }
                } else {
                    *self = Self::Idle;
                }
            }
            Self::Fire => {
                if has_target {
                    *self = Self::Targeting {
                        timer: Timer::from_seconds(1.0, TimerMode::Once),
                    };
                } else {
                    *self = Self::Idle;
                }
            }
        }
    }
}
