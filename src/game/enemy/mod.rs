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
        app.add_systems(Update, update_enemy);
        // Systems
        // On Exit State
        // .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
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

fn spawn_enemy(
    mut commands: Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
) {
    let asset = &asset_db.enemy_ship_1;

    commands
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
        });
}
