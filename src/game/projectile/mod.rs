use super::assets;
use super::assets::AssetDB;
use super::time_to_live::TimeToLive;
use super::vitality::*;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .insert(Damage(1))
        .insert(TimeToLive::from_seconds(3.0));
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_projectiles_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    // mut contact_force_events: EventReader<ContactForceEvent>,
    projectile_query: Query<(&Projectile, Option<&Damage>), Without<Health>>,
    mut health_query: Query<&mut Health, Without<Projectile>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            // Will be removed before collision is resolved
            CollisionEvent::Started(entity1, entity2, _) => {
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
    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {:?}", contact_force_event);
    // }
}

fn resolve_projectile_collision(
    commands: &mut Commands,
    projectile_query: &Query<(&Projectile, Option<&Damage>), Without<Health>>,
    health_query: &mut Query<&mut Health, Without<Projectile>>,

    entity1: &Entity,
    entity2: &Entity,
) -> bool {
    if let Ok((_, damge_opt)) = projectile_query.get(*entity1) {
        commands.entity(*entity1).despawn();
        if let Some(damage) = damge_opt {
            if let Ok(mut health) = health_query.get_mut(*entity2) {
                health.take_damage(damage)
            }
        }
        return true;
    }
    return false;
}
