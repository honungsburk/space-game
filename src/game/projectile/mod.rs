use super::assets;
use super::time_to_live::TimeToLive;
use super::vitality::*;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectiles_on_collision);
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

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////

pub fn despawn_projectiles(commands: &mut Commands, query: Query<Entity, With<Projectile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_laser_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
    collision_membership: &Group,
    collision_filter: &Group,
    damage: u32,
) {
    let laser_projectile = assets::PROJECTILE_LASER;

    commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(laser_projectile.sprite_path),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(laser_projectile.collider())
        .insert(CollisionGroups::new(
            (*collision_membership).into(),
            (*collision_filter).into(),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SolverGroups::new(
            (*collision_membership).into(),
            (*collision_filter).into(),
        ))
        .insert(Velocity {
            linvel: spawn_transform.rotation.mul_vec3(Vec3::Y).xy().normalize() * 1000.0,
            angvel: 0.0,
        })
        .insert(Projectile::new(ProjectileType::Laser))
        .insert(Damage(damage))
        .insert(TimeToLive::from_seconds(3.0));
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_projectiles_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(&Projectile, Option<&Damage>), Without<Health>>,
    mut health_query: Query<&mut Health, Without<Projectile>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            // Will be removed before collision is resolved
            CollisionEvent::Started(entity1, entity2, flags) => {
                if flags.contains(CollisionEventFlags::REMOVED) {
                    continue;
                }

                let did_resolve = resolve_projectile_collision(
                    &mut commands,
                    &projectile_query,
                    &mut health_query,
                    entity1,
                    entity2,
                );

                if !did_resolve {
                    resolve_projectile_collision(
                        &mut commands,
                        &projectile_query,
                        &mut health_query,
                        entity2,
                        entity1,
                    );
                }
            }
            _ => {}
        }
    }
}

fn resolve_projectile_collision(
    commands: &mut Commands,
    projectile_query: &Query<(&Projectile, Option<&Damage>), Without<Health>>,
    health_query: &mut Query<&mut Health, Without<Projectile>>,
    entity1: &Entity,
    entity2: &Entity,
) -> bool {
    if let Ok((_, damge_opt)) = projectile_query.get(*entity1) {
        commands.entity(*entity1).despawn_recursive();
        if let Some(damage) = damge_opt {
            if let Ok(mut health) = health_query.get_mut(*entity2) {
                health.take_damage(damage);
            }
        }
        return true;
    }
    return false;
}
