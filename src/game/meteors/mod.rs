use super::assets;
use super::assets::groups;
use super::assets::Asset;
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

pub fn meteor_asset(size: &MeteorSize, color: &MeteorColor) -> Asset {
    match (size, color) {
        (MeteorSize::Tiny, MeteorColor::Brown) => assets::METEOR_BROWN_TINY_1,
        (MeteorSize::Tiny, MeteorColor::Grey) => assets::METEOR_GREY_TINY_1,
        (MeteorSize::Small, MeteorColor::Brown) => assets::METEOR_BROWN_SMALL_1,
        (MeteorSize::Small, MeteorColor::Grey) => assets::METEOR_GREY_SMALL_1,
        (MeteorSize::Medium, MeteorColor::Brown) => assets::METEOR_BROWN_MEDIUM_1,
        (MeteorSize::Medium, MeteorColor::Grey) => assets::METEOR_GREY_MEDIUM_1,
        (MeteorSize::Big, MeteorColor::Brown) => assets::METEOR_BROWN_BIG_1,
        (MeteorSize::Big, MeteorColor::Grey) => assets::METEOR_GREY_BIG_1,
    }
}

pub fn spawn(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    size: MeteorSize,
    transform: Transform,
    linear_velocity: Vec2,
    angel_velocity: f32,
) {
    let asset = meteor_asset(&size, &MeteorColor::Brown);
    commands
        .spawn(SpriteBundle {
            transform: transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(Meteor)
        .insert(RigidBody::Dynamic)
        .insert(asset.collider())
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
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    size: MeteorSize,
    transform: Transform,
) {
    let asset = meteor_asset(&size, &MeteorColor::Grey);
    commands
        .spawn(SpriteBundle {
            transform: transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(CollisionGroups::new(
            groups::METEOR_GROUP.into(),
            groups::METEOR_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::METEOR_GROUP.into(),
            groups::METEOR_FILTER_MASK.into(),
        ))
        .insert(Meteor)
        .insert(RigidBody::Fixed)
        .insert(asset.collider());
}

pub fn despawn_all(commands: &mut Commands, arena_query: &Query<Entity, With<Meteor>>) {
    for entity in arena_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
