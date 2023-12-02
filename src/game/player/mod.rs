mod actions;
pub mod components;
mod systems;

use crate::game::average_velocity::AverageVelocity;
use crate::game::game_entity::GameEntityType;
use crate::game::thrustor::AngularThrustor;
use crate::game::trauma::Trauma;
use crate::game::vitality::Health;
use crate::game::{assets, assets::groups, weapon::Weapon};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::InputManagerBundle;
use systems::*;

pub use actions::PlayerShipAction;
pub use components::PlayerLabel;

use self::components::ContactForceInvulnerability;

use super::kamikaze_drone::KamikazeDroneTargetLabel;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerShipAction>::default())
            .add_systems(
                Update,
                (
                    control_ship,
                    fire_weapon,
                    player_collision,
                    update_contact_force_invulnerability,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Spawning
////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player_at_center(commands: Commands, asset_server: Res<AssetServer>) {
    spawn(Vec2::new(0.0, 0.0), std::f32::consts::PI / 2.0)(commands, asset_server);
}

pub fn spawn(location: Vec2, rotation: f32) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands, asset_server| {
        spawn_player(&mut commands, &asset_server, location, rotation);
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    location: Vec2,
    rotation: f32,
) -> Entity {
    // Spawn transform
    let spawn_transform = Transform::from_xyz(location.x, location.y, 0.0)
        .with_rotation(Quat::from_rotation_z(rotation));

    // Add the player entity

    commands
        .spawn(SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load(assets::PLAYER_SHIP.sprite_path),
            ..default()
        })
        .insert(PlayerLabel {})
        .insert(GameEntityType::Player)
        .insert(AngularThrustor::with_max_angular_acceleration(1.0))
        .insert(InputManagerBundle::<PlayerShipAction> {
            action_state: ActionState::default(),
            input_map: actions::create_input_map(),
        })
        .insert(RigidBody::Dynamic)
        .insert(assets::PLAYER_SHIP.collider())
        .insert(Trauma::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceInvulnerability::new(0.1))
        .insert(ContactForceEventThreshold(0.0)) // TODO: increase this to some reasonable value
        .insert(Health::at_max(100))
        .insert(CollisionGroups::new(
            groups::PLAYER_GROUP.into(),
            groups::PLAYER_FILTER_MASK.into(),
        ))
        .insert(SolverGroups::new(
            groups::PLAYER_GROUP.into(),
            groups::PLAYER_FILTER_MASK.into(),
        ))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(ReadMassProperties::default())
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Velocity::default())
        .insert(KamikazeDroneTargetLabel)
        .insert(AverageVelocity::new(0.5))
        .insert(Weapon::laser(
            10,
            1000.0,
            Timer::from_seconds(1.0, TimerMode::Once),
            Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
            groups::PLAYER_PROJECTILE_GROUP,
            groups::PLAYER_PROJECTILE_FILTER_MASK,
        ))
        .id()
}

pub fn despawn(mut commands: Commands, player_query: Query<Entity, With<PlayerLabel>>) {
    despawn_all(&mut commands, &player_query)
}

pub fn despawn_all(commands: &mut Commands, player_query: &Query<Entity, With<PlayerLabel>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}
