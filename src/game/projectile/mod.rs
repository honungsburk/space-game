use super::assets;
use super::assets::AssetDB;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_projectiles_on_collision, update_time_to_live),
        );
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

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_laser_projectile(
    mut commands: Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
) {
    let laser_projectile = &asset_db.laser_projectile;

    commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(laser_projectile.sprite_path),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(laser_projectile.collider.clone())
        .insert(CollisionGroups::new(
            assets::PLAYER_PROJECTILE_GROUP.into(),
            assets::PLAYER_PROJECTILE_FILTER_MASK.into(),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SolverGroups::new(
            assets::PLAYER_PROJECTILE_GROUP.into(),
            assets::PLAYER_PROJECTILE_FILTER_MASK.into(),
        ))
        .insert(Velocity {
            linvel: spawn_transform.rotation.mul_vec3(Vec3::Y).xy().normalize() * 1000.0,
            angvel: 0.0,
        })
        .insert(Projectile::new(ProjectileType::Laser))
        .insert(Damage(1.0))
        .insert(TimeToLive::from_seconds(3.0));
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_projectiles_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    query: Query<(Entity, &Projectile)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            // Will be removed before collision is resolved
            CollisionEvent::Started(entity1, entity2, _) => {
                if query.contains(*entity1) {
                    commands.entity(*entity1).despawn();
                }

                if query.contains(*entity2) {
                    commands.entity(*entity2).despawn();
                }
            }
            _ => {}
        }
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
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
