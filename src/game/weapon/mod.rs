use bevy::prelude::*;

use super::{assets::AssetDB, projectile as Projectile};

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_weapon));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Weapon(WeaponType);

pub enum WeaponType {
    Laser {
        projectile_damage: f32,
        projectile_speed: f32,
        projectile_time_to_live: Timer,
        cooldown: Timer,
    },
}

impl Weapon {
    pub fn new(weapon_type: WeaponType) -> Self {
        Self(weapon_type)
    }

    pub fn can_fire(&self) -> bool {
        match &self.0 {
            WeaponType::Laser {
                projectile_damage: _,
                projectile_speed: _,
                projectile_time_to_live: _,
                cooldown,
            } => cooldown.finished(),
        }
    }

    pub fn fire(
        &mut self,
        mut commands: Commands,
        asset_db: &Res<AssetDB>,
        asset_server: &Res<AssetServer>,
        spawn_transform: Transform,
    ) {
        Projectile::spawn_laser_projectile(commands, &asset_db, &asset_server, spawn_transform);

        match &mut self.0 {
            WeaponType::Laser {
                projectile_damage: _,
                projectile_speed: _,
                projectile_time_to_live: _,
                cooldown,
            } => cooldown.reset(),
        }
    }

    pub fn from_laser(
        projectile_damage: f32,
        projectile_speed: f32,
        projectile_time_to_live: Timer,
        cooldown: Timer,
    ) -> Self {
        Self(WeaponType::Laser {
            projectile_damage,
            projectile_speed,
            projectile_time_to_live,
            cooldown,
        })
    }

    pub fn simple_laser() -> Self {
        Self(WeaponType::Laser {
            projectile_damage: 1.0,
            projectile_speed: 1000.0,
            projectile_time_to_live: Timer::from_seconds(1.0, TimerMode::Once),
            cooldown: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_weapon(time: Res<Time>, mut query: Query<&mut Weapon>) {
    for mut weapon in query.iter_mut() {
        match &mut weapon.0 {
            WeaponType::Laser {
                projectile_damage: _,
                projectile_speed: _,
                projectile_time_to_live: _,
                cooldown,
            } => {
                cooldown.tick(time.delta());
            }
        }
    }
}
