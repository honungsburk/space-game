pub mod groups;

use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use std::f32::consts::PI;

////////////////////////////////////////////////////////////////////////////////
// Asset Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_asset_db());
    }
}

////////////////////////////////////////////////////////////////////////////////
// Asset Manager
////////////////////////////////////////////////////////////////////////////////

pub struct Asset {
    // pub id: usize,
    pub name: &'static str,
    pub sprite_path: &'static str,
    pub collider: Collider,
}

impl Asset {
    pub const fn new(name: &'static str, sprite_path: &'static str, collider: Collider) -> Self {
        Self {
            name,
            sprite_path,
            collider,
        }
    }
}
#[derive(Resource)]
pub struct AssetDB {
    // Player Assets
    pub player_ship: Asset,

    // Meteor Assets
    pub meteor_brown_big_1: Asset,
    pub meteor_brown_big_2: Asset,
    pub meteor_brown_big_3: Asset,
    pub meteor_brown_big_4: Asset,
    pub meteor_brown_medium_1: Asset,
    pub meteor_brown_medium_2: Asset,
    pub meteor_brown_small_1: Asset,
    pub meteor_brown_small_2: Asset,
    pub meteor_brown_tiny_1: Asset,
    pub meteor_brown_tiny_2: Asset,
    pub meteor_grey_big_1: Asset,
    pub meteor_grey_big_2: Asset,
    pub meteor_grey_big_3: Asset,
    pub meteor_grey_big_4: Asset,
    pub meteor_grey_medium_1: Asset,
    pub meteor_grey_medium_2: Asset,
    pub meteor_grey_small_1: Asset,
    pub meteor_grey_small_2: Asset,
    pub meteor_grey_tiny_1: Asset,
    pub meteor_grey_tiny_2: Asset,

    // Turret Assets
    pub turret_base_big: Asset,
    pub turret_base_small: Asset,

    // Guns
    pub gun_8: Asset,

    // Projectile Assets
    pub laser_projectile: Asset,

    // Enemy Assets
    pub enemy_ship_1: Asset,
    pub kamikaze_drone: Asset,
}

////////////////////////////////////////////////////////////////////////////////
// AssetDB
////////////////////////////////////////////////////////////////////////////////

pub const BIG_METEOR_RADIUS: f32 = 50.0;
pub const MEDIUM_METEOR_RADIUS: f32 = 25.0;
pub const SMALL_METEOR_RADIUS: f32 = 12.5;
pub const TINY_METEOR_RADIUS: f32 = 6.25;

fn ship_collider() -> Collider {
    Collider::compound(vec![
        // Main Body
        (Vec2::ZERO, 0.0, Collider::round_cuboid(6.0, 34.0, 0.05)),
        // Wing Left
        (
            Vec2::Y * -10.0,
            PI / 2.0,
            Collider::round_convex_hull(
                &[
                    Vec2::new(20.0, 0.0),
                    Vec2::new(-24.0, 0.0),
                    Vec2::new(12.0, 44.0),
                    Vec2::new(-8.0, 44.0),
                ],
                0.05,
            )
            .unwrap(),
        ),
        // Wing Right
        (
            Vec2::Y * -10.0,
            -PI / 2.0,
            Collider::round_convex_hull(
                &[
                    Vec2::new(24.0, 0.0),
                    Vec2::new(-20.0, 0.0),
                    Vec2::new(8.0, 44.0),
                    Vec2::new(-12.0, 44.0),
                ],
                0.05,
            )
            .unwrap(),
        ),
    ])
}

// Assets
fn create_asset_db() -> AssetDB {
    AssetDB {
        player_ship: Asset::new(
            "Player Ship",
            "sprites/playerShip1_blue.png",
            ship_collider(),
        ),
        meteor_brown_big_1: Asset::new(
            "Meteor Brown Big 1",
            "sprites/meteors/meteorBrown_big1.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_brown_big_2: Asset::new(
            "Meteor Brown Big 2",
            "sprites/meteors/meteorBrown_big2.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_brown_big_3: Asset::new(
            "Meteor Brown Big 3",
            "sprites/meteors/meteorBrown_big3.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_brown_big_4: Asset::new(
            "Meteor Brown Big 4",
            "sprites/meteors/meteorBrown_big4.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_brown_medium_1: Asset::new(
            "Meteor Brown Med 1",
            "sprites/meteors/meteorBrown_med1.png",
            Collider::ball(MEDIUM_METEOR_RADIUS),
        ),
        meteor_brown_medium_2: Asset::new(
            "Meteor Brown Med 2",
            "sprites/meteors/meteorBrown_med2.png",
            Collider::ball(MEDIUM_METEOR_RADIUS),
        ),
        meteor_brown_small_1: Asset::new(
            "Meteor Brown Small 1",
            "sprites/meteors/meteorBrown_small1.png",
            Collider::ball(SMALL_METEOR_RADIUS),
        ),
        meteor_brown_small_2: Asset::new(
            "Meteor Brown Small 2",
            "sprites/meteors/meteorBrown_small2.png",
            Collider::ball(SMALL_METEOR_RADIUS),
        ),
        meteor_brown_tiny_1: Asset::new(
            "Meteor Brown Tiny 1",
            "sprites/meteors/meteorBrown_tiny1.png",
            Collider::ball(TINY_METEOR_RADIUS),
        ),
        meteor_brown_tiny_2: Asset::new(
            "Meteor Brown Tiny 2",
            "sprites/meteors/meteorBrown_tiny2.png",
            Collider::ball(TINY_METEOR_RADIUS),
        ),
        meteor_grey_big_1: Asset::new(
            "Meteor Grey Big 1",
            "sprites/meteors/meteorGrey_big1.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_grey_big_2: Asset::new(
            "Meteor Grey Big 2",
            "sprites/meteors/meteorGrey_big2.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_grey_big_3: Asset::new(
            "Meteor Grey Big 3",
            "sprites/meteors/meteorGrey_big3.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_grey_big_4: Asset::new(
            "Meteor Grey Big 4",
            "sprites/meteors/meteorGrey_big4.png",
            Collider::ball(BIG_METEOR_RADIUS),
        ),
        meteor_grey_medium_1: Asset::new(
            "Meteor Grey Med 1",
            "sprites/meteors/meteorGrey_med1.png",
            Collider::ball(MEDIUM_METEOR_RADIUS),
        ),
        meteor_grey_medium_2: Asset::new(
            "Meteor Grey Med 2",
            "sprites/meteors/meteorGrey_med2.png",
            Collider::ball(MEDIUM_METEOR_RADIUS),
        ),
        meteor_grey_small_1: Asset::new(
            "Meteor Grey Small 1",
            "sprites/meteors/meteorGrey_small1.png",
            Collider::ball(SMALL_METEOR_RADIUS),
        ),
        meteor_grey_small_2: Asset::new(
            "Meteor Grey Small 2",
            "sprites/meteors/meteorGrey_small2.png",
            Collider::ball(SMALL_METEOR_RADIUS),
        ),
        meteor_grey_tiny_1: Asset::new(
            "Meteor Grey Tiny 1",
            "sprites/meteors/meteorGrey_tiny1.png",
            Collider::ball(TINY_METEOR_RADIUS),
        ),
        meteor_grey_tiny_2: Asset::new(
            "Meteor Grey Tiny 2",
            "sprites/meteors/meteorGrey_tiny2.png",
            Collider::ball(TINY_METEOR_RADIUS),
        ),

        // Projectiles
        laser_projectile: Asset::new(
            "Laser Projectile",
            "sprites/laserBlue01.png",
            Collider::capsule_y(22.0, 5.0),
        ),

        // Gun
        gun_8: Asset::new(
            "Gun 8",
            "sprites/parts/gun/gun08.png",
            Collider::cuboid(5.0, 15.0),
        ),

        // Turret Base
        turret_base_big: Asset::new(
            "Turret Base Big",
            "sprites/parts/turret/turretBase_big.png",
            Collider::ball(20.0),
        ),

        turret_base_small: Asset::new(
            "Turret Base Small",
            "sprites/parts/turret/turretBase_small.png",
            Collider::ball(15.0),
        ),

        // Enemy Assets
        enemy_ship_1: Asset::new(
            "Enemy Ship 1",
            "sprites/enemy/enemyRed1.png",
            Collider::ball(50.0),
        ),

        kamikaze_drone: Asset::new(
            "Kamikaze Drone",
            "sprites/enemy/kamikaze_drone.png",
            Collider::ball(10.0),
        ),
    }
}
