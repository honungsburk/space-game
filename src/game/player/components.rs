use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub enum Weapon {
    Laser {
        projectile_damage: f32,
        projectile_speed: f32,
        projectile_time_to_live: Timer,
        cooldown: Timer,
    },
}
