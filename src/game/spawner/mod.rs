use std::time::Duration;

use crate::misc::random;

use super::{screen_bounds::ScreenBounds, simple_enemy};
use bevy::prelude::*;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

#[derive(Component, Debug)]
pub struct Spawner {
    timer: Timer,
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Spawner>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

const MAX_SPAWN_COUNT: usize = 1000;

fn update(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    time: Res<Time>,
    screen_bounds: Res<ScreenBounds>,
    enemies: Query<Entity, With<simple_enemy::SimpleEnemyLabel>>,
    mut query: Query<&mut Spawner>,
) {
    let number_of_enemies = enemies.iter().count();

    for mut spawner in query.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            let new_duration = spawner
                .timer
                .duration()
                .mul_f32(0.99)
                .max(Duration::from_secs_f32(0.1));
            spawner.timer.set_duration(new_duration);

            if number_of_enemies > MAX_SPAWN_COUNT {
                continue;
            }

            let mut rng = rand::thread_rng();
            let radius = screen_bounds.half_diagonal();

            let center = screen_bounds.center();

            let location = center + random::uniform_donut(&mut rng, radius * 1.5, radius * 1.1);

            let rotation = center.angle_between(location);

            simple_enemy::spawn(&mut commands, &mut asset_server, location, rotation);
        }
    }
}
