//! # Enemy Ship
//!
//! ## Behavior
//!
//! The movement works with attraction and repulsion forces. The enemy ship is
//! attracted to the player ship and repulsed non-player entities. The enemy sees in a cone
//! in front of it. If the player is in the cone, the enemy will move towards the player.
//!
//! **Rules:**
//! - Moves forward
//! - If there are more then one enemy entity, move towards the first one that was seen (the influence is constant).
//! - If an entity is in the view cone (includes enemies), move away from it (the influence: -1 / distance^2).
//! - If there is an enemy entity in the view cone, shoot.
//!
//!

mod ai;

use super::assets::groups;
use super::assets::AssetDB;
use super::game_entity::Enemy;
use super::game_entity::GameEntityType;
use super::player::components::Player;
use super::vitality::Health;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use bevy_rapier2d::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, update_enemy);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_enemy(
    mut enemy_query: Query<(&mut ExternalImpulse, &mut Transform), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (mut external_impulse, mut transform) in enemy_query.iter_mut() {
            let distance_to_player = player_transform.translation - transform.translation;

            transform.rotation =
                Quat::from_rotation_z(Vec2::Y.angle_between(distance_to_player.xy()));

            let offset = 200.0;

            if distance_to_player.length() > offset - 10.0
                && distance_to_player.length() < offset + 10.0
            {
                continue;
            }

            let mut impulse = distance_to_player.normalize() * 0.7;

            if distance_to_player.length() < offset - 10.0 {
                impulse = -1.0 * impulse;
            }
            external_impulse.impulse = impulse.xy();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Spawn
////////////////////////////////////////////////////////////////////////////////

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn(
    commands: &mut Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    spawn_location: Vec2,
    rotation: f32,
) -> Entity {
    let asset = &asset_db.enemy_ship_1;

    let mut spawn_transform = Transform::from_translation(spawn_location.extend(0.0));
    spawn_transform.rotate_local_z(rotation);

    let entity = commands
        .spawn(SpriteBundle {
            texture: asset_server.load(asset.sprite_path),
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                // And don't flip it upside-down ( the default )
                flip_y: true,
                ..default()
            },
            transform: spawn_transform,
            ..Default::default()
        })
        .insert(GameEntityType::Enemy)
        .insert(Enemy)
        .insert(Health::at_max(1))
        .insert(RigidBody::Dynamic)
        .insert(asset.collider.clone())
        .insert(CollisionGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::ENEMY_GROUP.into(),
            groups::ENEMY_FILTER_MASK.into(),
        ))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .id();

    return entity;
}
