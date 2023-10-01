use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::distributions::*;
use rand::prelude::*;

use super::assets::Asset;
use super::assets::AssetDB;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_random_meteor)
            .add_systems(Update, update_meteors);
    }
}

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
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
    commands: &mut Commands,
    size: MeteorSize,
    color: MeteorColor,
    transform: Transform,
    linear_velocity: Vec2,
    angel_velocity: f32,
) {
    let asset = meteor_asset(&asset_db, &size, &color);
    commands
        .spawn(SpriteBundle {
            transform: transform,
            texture: asset_server.load(asset.sprite_path),
            ..Default::default()
        })
        .insert(Meteor)
        .insert(RigidBody::Dynamic)
        .insert(asset.collider.clone())
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

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_random_meteor(
    mut commands: Commands,
    asset_db: Res<AssetDB>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    let uniform = Uniform::new(0.0, 1.0);

    let x: f32 = uniform.sample(&mut rng) * window.width();
    let y: f32 = uniform.sample(&mut rng) * window.height();
    // Only positive runbers!
    // Make sure not to spawn on player!
    let transform = Transform::from_xyz(200.0, 100.0, 0.0);

    println!(
        "Spawning meteor at: {:?}, {:?}",
        x.to_string().as_str(),
        y.to_string().as_str()
    );

    spawn_meteor(
        asset_db,
        asset_server,
        &mut commands,
        MeteorSize::Big,
        MeteorColor::Brown,
        transform,
        Vec2::ZERO,
        0.0,
    );
}

pub fn update_meteors() {}
