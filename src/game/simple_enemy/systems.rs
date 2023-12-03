use crate::game::{
    player::PlayerLabel,
    thrustor::{AngularThrustor, LinearThrustor},
    vitality::{Damage, Health},
};

use super::components::SimpleEnemyLabel;
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

const MAX_SPEED: f32 = 200.0;

// drones: https://vanhunteradams.com/Pico/Animal_Movement/drones-algorithm.html
pub fn update(
    player_query: Query<&Transform, (With<PlayerLabel>, Without<SimpleEnemyLabel>)>,
    mut enemy_query: Query<
        (&Transform, &mut AngularThrustor, &mut LinearThrustor),
        (With<SimpleEnemyLabel>, Without<PlayerLabel>),
    >,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut angular_thrustor, mut linear_thrustor) in enemy_query.iter_mut() {
            let diff = (player_transform.translation - enemy_transform.translation).truncate();

            let target_velocity = diff.normalize() * MAX_SPEED;

            let target_angle = Vec2::Y.angle_between(target_velocity);

            linear_thrustor.set_desired_velocity(target_velocity);
            angular_thrustor.set_desired_angle(target_angle);
        }
    }
}

pub fn update_on_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut simple_enemy_query: Query<
        (&Damage, &mut Health),
        (Without<PlayerLabel>, With<SimpleEnemyLabel>),
    >,
    mut player_query: Query<
        (&mut Health, &Transform),
        (With<PlayerLabel>, Without<SimpleEnemyLabel>),
    >,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            // Will be removed before collision is resolved
            CollisionEvent::Started(entity1, entity2, flags) => {
                if flags.contains(CollisionEventFlags::REMOVED) {
                    continue;
                }

                let did_resolve = resolve_projectile_collision(
                    &mut simple_enemy_query,
                    &mut player_query,
                    entity1,
                    entity2,
                );

                if !did_resolve {
                    resolve_projectile_collision(
                        &mut simple_enemy_query,
                        &mut player_query,
                        entity2,
                        entity1,
                    );
                }
            }
            _ => {}
        }
    }
}

fn resolve_projectile_collision(
    simple_enemy_query: &mut Query<
        (&Damage, &mut Health),
        (Without<PlayerLabel>, With<SimpleEnemyLabel>),
    >,
    player_query: &mut Query<
        (&mut Health, &Transform),
        (With<PlayerLabel>, Without<SimpleEnemyLabel>),
    >,
    entity1: &Entity,
    entity2: &Entity,
) -> bool {
    if let Ok((damage, mut enemy_health)) = simple_enemy_query.get_mut(*entity1) {
        if let Ok((mut player_health, transform)) = player_query.get_mut(*entity2) {
            player_health.take_damage(damage);
            enemy_health.kill();
            return true;
        } else {
            return false;
        }
    }
    return false;
}
