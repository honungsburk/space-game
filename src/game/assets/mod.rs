use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use bevy_rapier2d::prelude::*;
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

    // Projectile Assets
    pub laser_projectile: Asset,
}

////////////////////////////////////////////////////////////////////////////////
// AssetDB
////////////////////////////////////////////////////////////////////////////////

pub const BIG_METEOR_RADIUS: f32 = 50.0;
pub const MEDIUM_METEOR_RADIUS: f32 = 25.0;
pub const SMALL_METEOR_RADIUS: f32 = 12.5;
pub const TINY_METEOR_RADIUS: f32 = 6.25;

// SOLVER GROUPS
pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const PLAYER_PROJECTILE_GROUP: Group = Group::GROUP_2;
pub const METEOR_GROUP: Group = Group::GROUP_3;
pub const ARENA_GROUP: Group = Group::GROUP_4;

pub const PLAYER_FILTER_MASK: Group = METEOR_GROUP.union(ARENA_GROUP);
pub const METEOR_FILTER_MASK: Group = PLAYER_GROUP
    .union(METEOR_GROUP)
    .union(PLAYER_PROJECTILE_GROUP)
    .union(ARENA_GROUP);
pub const PLAYER_PROJECTILE_FILTER_MASK: Group = METEOR_GROUP.union(ARENA_GROUP);
pub const ARENA_FILTER_MASK: Group = PLAYER_GROUP.union(METEOR_GROUP);

fn ship_collider() -> Collider {
    Collider::compound(vec![
        // Main Body
        (Vec2::ZERO, 0.0, Collider::cuboid(10.0, 38.0)),
        // Wing Left
        (
            Vec2::Y * -10.0,
            PI / 2.0,
            Collider::triangle(
                Vec2::new(30.0, 0.0),
                Vec2::new(-30.0, 0.0),
                Vec2::new(0.0, 48.0),
            ),
        ),
        (
            Vec2::new(-48.0, -10.0),
            -PI / 2.0,
            Collider::triangle(
                Vec2::new(14.0, 0.0),
                Vec2::new(-14.0, 0.0),
                Vec2::new(0.0, 20.0),
            ),
        ),
        // Wing Right
        (
            Vec2::Y * -10.0,
            -PI / 2.0,
            Collider::triangle(
                Vec2::new(30.0, 0.0),
                Vec2::new(-30.0, 0.0),
                Vec2::new(0.0, 48.0),
            ),
        ),
        (
            Vec2::new(48.0, -10.0),
            PI / 2.0,
            Collider::triangle(
                Vec2::new(14.0, 0.0),
                Vec2::new(-14.0, 0.0),
                Vec2::new(0.0, 20.0),
            ),
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

        laser_projectile: Asset::new(
            "Laser Projectile",
            "sprites/laserBlue01.png",
            Collider::capsule_y(22.0, 5.0),
        ),
    }
}
