use bevy::prelude::*;

pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update);
    }
}

// Time To Live

#[derive(Component)]
pub struct TimeToLive(pub Timer);

impl TimeToLive {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
    pub fn from_seconds(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.0.tick(time.delta());
        if ttl.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
