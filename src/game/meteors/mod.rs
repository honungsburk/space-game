use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyVelocity};
use rand::distributions::*;
use rand::prelude::*;

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

pub fn meteor_asset_path(size: MeteorSize, color: MeteorColor) -> &'static str {
    match (size, color) {
        (MeteorSize::Tiny, MeteorColor::Brown) => "sprites/meteors/meteorBrown_tiny1.png",
        (MeteorSize::Tiny, MeteorColor::Grey) => "sprites/meteors/meteorGrey_tiny1.png",
        (MeteorSize::Small, MeteorColor::Brown) => "sprites/meteors/meteorBrown_small1.png",
        (MeteorSize::Small, MeteorColor::Grey) => "sprites/meteors/meteorGrey_small1.png",
        (MeteorSize::Medium, MeteorColor::Brown) => "sprites/meteors/meteorBrown_med1.png",
        (MeteorSize::Medium, MeteorColor::Grey) => "sprites/meteors/meteorGrey_med1.png",
        (MeteorSize::Big, MeteorColor::Brown) => "sprites/meteors/meteorBrown_big1.png",
        (MeteorSize::Big, MeteorColor::Grey) => "sprites/meteors/meteorGrey_big1.png",
    }
}

pub fn spawn_meteor(
    asset_server: Res<AssetServer>,
    commands: &mut Commands,
    size: MeteorSize,
    color: MeteorColor,
    transform: Transform,
    linear_velocity: Vec2,
    angel_velocity: f32,
) {
    let texture = asset_server.load(meteor_asset_path(size, color));

    commands
        .spawn(SpriteBundle {
            transform: transform,
            texture,
            ..Default::default()
        })
        .insert(Meteor)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
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
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    let uniform = Uniform::new(-1.0, 1.0);

    let x: f32 = uniform.sample(&mut rng) * window.width() / 2.0;
    let y: f32 = uniform.sample(&mut rng) * window.height() / 2.0;
    let transform = Transform::from_xyz(x, y, 0.0);

    spawn_meteor(
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
