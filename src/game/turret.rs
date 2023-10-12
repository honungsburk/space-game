use std::f32::consts::PI;

use super::{
    assets::{self, AssetDB},
    components::Health,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    geometry::*,
    prelude::{ExternalForce, ExternalImpulse, RigidBody},
};
pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_turret);
        // Systems
        // On Exit State
        // .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}

////////////////////////////////////////////////////////////////////////////////
///
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Turret;

fn create_turret(
    commands: &mut Commands,
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
) {
    let turret_base = &asset_db.turret_base_big;
    let gun = &asset_db.gun_8;

    commands
        .spawn(Turret)
        // Properties
        .insert(Health::new(1))
        // Physics
        .insert(SpatialBundle::from_transform(spawn_transform))
        .insert(RigidBody::Dynamic)
        .insert(CollisionGroups::new(
            assets::ENEMY_GROUP.into(),
            assets::ENEMY_FILTER_MASK.into(),
        ))
        .insert(turret_base.collider.clone())
        .insert(SolverGroups::new(
            assets::ENEMY_GROUP.into(),
            assets::ENEMY_FILTER_MASK.into(),
        ))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .with_children(|parent| {
            let mut gun_transform = Transform::from_translation(Vec3::new(0.0, 20.0, 0.0));

            gun_transform.rotate(Quat::from_rotation_z(PI));

            parent.spawn(SpriteBundle {
                texture: asset_server.load(gun.sprite_path),
                transform: gun_transform,
                ..Default::default()
            });
            parent.spawn(SpriteBundle {
                texture: asset_server.load(turret_base.sprite_path),
                ..Default::default()
            });
            // .insert(SpatialBundle::default());

            // .insert(SpatialBundle::default());
        });

    // .add_child(turret_base_entity)
    // .add_child(turret_gun_entity);
}

fn spawn_turret(
    mut commands: Commands,
    asset_db: Res<crate::game::assets::AssetDB>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let arena_center = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);

    let spawn_transform = Transform::from_translation(arena_center + Vec3::new(0.0, 300.0, 0.0));

    create_turret(&mut commands, &asset_db, &asset_server, spawn_transform);
}
