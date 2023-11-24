use bevy_rapier2d::geometry::Group;

// Entity Collision Groups
pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const PLAYER_PROJECTILE_GROUP: Group = Group::GROUP_2;
pub const METEOR_GROUP: Group = Group::GROUP_3;
pub const ARENA_GROUP: Group = Group::GROUP_4;
pub const ENEMY_GROUP: Group = Group::GROUP_5;
pub const ENEMY_PROJECTILE_GROUP: Group = Group::GROUP_6;
pub const KAMIKAZE_DRONE_GROUP: Group = Group::GROUP_7;
pub const SENSOR_GROUP: Group = Group::GROUP_8;

// Entity Collision Filters
pub const PLAYER_FILTER_MASK: Group = METEOR_GROUP
    .union(ARENA_GROUP)
    .union(ENEMY_GROUP)
    .union(SENSOR_GROUP)
    .union(ENEMY_PROJECTILE_GROUP);
pub const METEOR_FILTER_MASK: Group = PLAYER_GROUP
    .union(METEOR_GROUP)
    .union(PLAYER_PROJECTILE_GROUP)
    .union(ENEMY_PROJECTILE_GROUP)
    .union(SENSOR_GROUP)
    .union(ARENA_GROUP)
    .union(ENEMY_GROUP);
pub const PLAYER_PROJECTILE_FILTER_MASK: Group = METEOR_GROUP.union(ARENA_GROUP).union(ENEMY_GROUP);
pub const ARENA_FILTER_MASK: Group = PLAYER_GROUP.union(METEOR_GROUP);
pub const ENEMY_FILTER_MASK: Group = PLAYER_GROUP
    .union(METEOR_GROUP)
    .union(SENSOR_GROUP)
    .union(ENEMY_GROUP)
    .union(PLAYER_PROJECTILE_GROUP);
pub const ENEMY_PROJECTILE_FILTER_MASK: Group = METEOR_GROUP.union(PLAYER_GROUP);
pub const KAMIKAZE_DRONE_FILTER_MASK: Group = SENSOR_GROUP;
