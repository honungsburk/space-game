use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectiles, update_time_to_live));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Projectile(ProjectileType);

impl Projectile {
    pub fn new(projectile_type: ProjectileType) -> Self {
        Self(projectile_type)
    }
}

pub enum ProjectileType {
    Laser,
}

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct TimeToLive(Timer);

impl TimeToLive {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
    pub fn from_seconds(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct Speed(pub f32);

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_laser_projectile(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
) {
    commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load("sprites/laserBlue01.png"),
            ..default()
        })
        .insert(Projectile::new(ProjectileType::Laser))
        .insert(Speed::new(500.0))
        .insert(Damage(1.0))
        .insert(TimeToLive::from_seconds(3.0));
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn update_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<Projectile>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        let step = speed.0 * time.delta_seconds() * transform.rotation.mul_vec3(Vec3::Y);
        transform.translation += step;
    }
}

pub fn update_time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.0.tick(time.delta());
        if ttl.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
