use super::assets::groups;
use super::assets::Asset;
use super::assets::AssetDB;
use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use bevy_rapier2d::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Meteor;

pub enum MeteorSize {
    Tiny,
    Small,
    Medium,
    Big,
}

pub enum MeteorColor {
    Brown,
    Grey,
}

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////

pub fn meteor_asset<'a>(
    asset_db: &'a AssetDB,
    size: &MeteorSize,
    color: &MeteorColor,
) -> &'a Asset {
    match (size, color) {
        (MeteorSize::Tiny, MeteorColor::Brown) => &asset_db.meteor_brown_tiny_1,
        (MeteorSize::Tiny, MeteorColor::Grey) => &asset_db.meteor_grey_tiny_1,
        (MeteorSize::Small, MeteorColor::Brown) => &asset_db.meteor_brown_small_1,
        (MeteorSize::Small, MeteorColor::Grey) => &asset_db.meteor_grey_small_1,
        (MeteorSize::Medium, MeteorColor::Brown) => &asset_db.meteor_brown_medium_1,
        (MeteorSize::Medium, MeteorColor::Grey) => &asset_db.meteor_grey_medium_1,
        (MeteorSize::Big, MeteorColor::Brown) => &asset_db.meteor_brown_big_1,
        (MeteorSize::Big, MeteorColor::Grey) => &asset_db.meteor_grey_big_1,
    }
}

pub fn spawn_meteor(
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    size: MeteorSize,
    transform: Transform,
    linear_velocity: Vec2,
    angel_velocity: f32,
) {
    let asset = meteor_asset(&asset_db, &size, &MeteorColor::Brown);
    commands
        .spawn(SpriteBundle {
            transform: transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(Meteor)
        .insert(RigidBody::Dynamic)
        .insert(asset.collider.clone())
        .insert(ColliderMassProperties::Density(2.0))
        .insert(CollisionGroups::new(
            groups::METEOR_GROUP.into(),
            groups::METEOR_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::METEOR_GROUP.into(),
            groups::METEOR_FILTER_MASK.into(),
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
        .insert(Velocity {
            linvel: linear_velocity,
            angvel: angel_velocity,
        });
}

pub fn spawn_immovable_meteor(
    asset_db: &Res<AssetDB>,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    size: MeteorSize,
    transform: Transform,
) {
    let asset = meteor_asset(&asset_db, &size, &MeteorColor::Grey);
    commands
        .spawn(SpriteBundle {
            transform: transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(Meteor)
        .insert(RigidBody::Fixed)
        .insert(asset.collider.clone());
}

pub fn despawn_all(commands: &mut Commands, arena_query: &Query<Entity, With<Meteor>>) {
    for entity in arena_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
