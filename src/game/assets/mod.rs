pub mod groups;

use bevy::prelude::*;
use bevy_rapier2d::geometry::*;
use std::f32::consts::PI;

////////////////////////////////////////////////////////////////////////////////
// Asset
////////////////////////////////////////////////////////////////////////////////

pub struct Asset {
    pub name: &'static str,
    pub sprite_path: &'static str,
    pub create_collider: fn() -> Collider,
}

impl Asset {
    pub const fn new(
        name: &'static str,
        sprite_path: &'static str,
        create_collider: fn() -> Collider,
    ) -> Self {
        Self {
            name,
            sprite_path,
            create_collider,
        }
    }

    pub fn collider(&self) -> Collider {
        (self.create_collider)()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Assets
////////////////////////////////////////////////////////////////////////////////

pub const ALL: [&'static [&'static Asset]; 6] = [
    &ALL_PLAYERS,
    &ALL_METEORS,
    &ALL_TURRET_BASES,
    &ALL_GUNS,
    &ALL_PROJECTILES,
    &ALL_ENEMIES,
];

// PLAYER

pub const ALL_PLAYERS: [&'static Asset; 1] = [&PLAYER_SHIP];

pub const PLAYER_SHIP: Asset =
    Asset::new("Player Ship", "sprites/playerShip1_blue.png", ship_collider);

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

// METEOR

pub const ALL_METEORS: [&'static Asset; 20] = [
    &METEOR_BROWN_BIG_1,
    &METEOR_BROWN_BIG_2,
    &METEOR_BROWN_BIG_3,
    &METEOR_BROWN_BIG_4,
    &METEOR_BROWN_MEDIUM_1,
    &METEOR_BROWN_MEDIUM_2,
    &METEOR_BROWN_SMALL_1,
    &METEOR_BROWN_SMALL_2,
    &METEOR_BROWN_TINY_1,
    &METEOR_BROWN_TINY_2,
    &METEOR_GREY_BIG_1,
    &METEOR_GREY_BIG_2,
    &METEOR_GREY_BIG_3,
    &METEOR_GREY_BIG_4,
    &METEOR_GREY_MEDIUM_1,
    &METEOR_GREY_MEDIUM_2,
    &METEOR_GREY_SMALL_1,
    &METEOR_GREY_SMALL_2,
    &METEOR_GREY_TINY_1,
    &METEOR_GREY_TINY_2,
];

pub const METEOR_BIG_RADIUS: f32 = 50.0;
pub const METEOR_MEDIUM_RADIUS: f32 = 25.0;
pub const METEOR_SMALL_RADIUS: f32 = 12.5;
pub const METEOR_TINY_RADIUS: f32 = 6.25;

pub const METEOR_BROWN_BIG_1: Asset = Asset::new(
    "Meteor Brown Big 1",
    "sprites/meteors/meteorBrown_big1.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);

pub const METEOR_BROWN_BIG_2: Asset = Asset::new(
    "Meteor Brown Big 2",
    "sprites/meteors/meteorBrown_big2.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_BROWN_BIG_3: Asset = Asset::new(
    "Meteor Brown Big 3",
    "sprites/meteors/meteorBrown_big3.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_BROWN_BIG_4: Asset = Asset::new(
    "Meteor Brown Big 4",
    "sprites/meteors/meteorBrown_big4.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_BROWN_MEDIUM_1: Asset = Asset::new(
    "Meteor Brown Med 1",
    "sprites/meteors/meteorBrown_med1.png",
    || Collider::ball(METEOR_MEDIUM_RADIUS),
);
pub const METEOR_BROWN_MEDIUM_2: Asset = Asset::new(
    "Meteor Brown Med 2",
    "sprites/meteors/meteorBrown_med2.png",
    || Collider::ball(METEOR_MEDIUM_RADIUS),
);
pub const METEOR_BROWN_SMALL_1: Asset = Asset::new(
    "Meteor Brown Small 1",
    "sprites/meteors/meteorBrown_small1.png",
    || Collider::ball(METEOR_SMALL_RADIUS),
);
pub const METEOR_BROWN_SMALL_2: Asset = Asset::new(
    "Meteor Brown Small 2",
    "sprites/meteors/meteorBrown_small2.png",
    || Collider::ball(METEOR_SMALL_RADIUS),
);
pub const METEOR_BROWN_TINY_1: Asset = Asset::new(
    "Meteor Brown Tiny 1",
    "sprites/meteors/meteorBrown_tiny1.png",
    || Collider::ball(METEOR_TINY_RADIUS),
);
pub const METEOR_BROWN_TINY_2: Asset = Asset::new(
    "Meteor Brown Tiny 2",
    "sprites/meteors/meteorBrown_tiny2.png",
    || Collider::ball(METEOR_TINY_RADIUS),
);
pub const METEOR_GREY_BIG_1: Asset = Asset::new(
    "Meteor Grey Big 1",
    "sprites/meteors/meteorGrey_big1.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_GREY_BIG_2: Asset = Asset::new(
    "Meteor Grey Big 2",
    "sprites/meteors/meteorGrey_big2.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_GREY_BIG_3: Asset = Asset::new(
    "Meteor Grey Big 3",
    "sprites/meteors/meteorGrey_big3.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_GREY_BIG_4: Asset = Asset::new(
    "Meteor Grey Big 4",
    "sprites/meteors/meteorGrey_big4.png",
    || Collider::ball(METEOR_BIG_RADIUS),
);
pub const METEOR_GREY_MEDIUM_1: Asset = Asset::new(
    "Meteor Grey Med 1",
    "sprites/meteors/meteorGrey_med1.png",
    || Collider::ball(METEOR_MEDIUM_RADIUS),
);
pub const METEOR_GREY_MEDIUM_2: Asset = Asset::new(
    "Meteor Grey Med 2",
    "sprites/meteors/meteorGrey_med2.png",
    || Collider::ball(METEOR_MEDIUM_RADIUS),
);
pub const METEOR_GREY_SMALL_1: Asset = Asset::new(
    "Meteor Grey Small 1",
    "sprites/meteors/meteorGrey_small1.png",
    || Collider::ball(METEOR_SMALL_RADIUS),
);
pub const METEOR_GREY_SMALL_2: Asset = Asset::new(
    "Meteor Grey Small 2",
    "sprites/meteors/meteorGrey_small2.png",
    || Collider::ball(METEOR_SMALL_RADIUS),
);
pub const METEOR_GREY_TINY_1: Asset = Asset::new(
    "Meteor Grey Tiny 1",
    "sprites/meteors/meteorGrey_tiny1.png",
    || Collider::ball(METEOR_TINY_RADIUS),
);
pub const METEOR_GREY_TINY_2: Asset = Asset::new(
    "Meteor Grey Tiny 2",
    "sprites/meteors/meteorGrey_tiny2.png",
    || Collider::ball(METEOR_TINY_RADIUS),
);

// TURRET

pub const ALL_TURRET_BASES: [&'static Asset; 2] = [&TURRET_BASE_BIG, &TURRET_BASE_SMALL];

pub const TURRET_BASE_BIG: Asset = Asset::new(
    "Turret Base Big",
    "sprites/parts/turret/turretBase_big.png",
    || Collider::ball(20.0),
);

pub const TURRET_BASE_SMALL: Asset = Asset::new(
    "Turret Base Small",
    "sprites/parts/turret/turretBase_small.png",
    || Collider::ball(15.0),
);

// GUN

pub const ALL_GUNS: [&'static Asset; 1] = [&GUN_8];

pub const GUN_8: Asset = Asset::new("Gun 8", "sprites/parts/gun/gun08.png", || {
    Collider::cuboid(5.0, 15.0)
});

// Projectiles

pub const ALL_PROJECTILES: [&'static Asset; 1] = [&PROJECTILE_LASER];

pub const PROJECTILE_LASER: Asset =
    Asset::new("Laser Projectile", "sprites/laserBlue01.png", || {
        Collider::capsule_y(22.0, 5.0)
    });

// ENEMY

pub const ALL_ENEMIES: [&'static Asset; 2] = [&ENEMY_SHIP_1, &KAMIKAZE_DRONE];

pub const ENEMY_SHIP_1: Asset = Asset::new("Enemy Ship 1", "sprites/enemy/enemyRed1.png", || {
    Collider::ball(50.0)
});

pub const KAMIKAZE_DRONE: Asset =
    Asset::new("Kamikaze Drone", "sprites/enemy/kamikaze_drone.png", || {
        Collider::ball(10.0)
    });
