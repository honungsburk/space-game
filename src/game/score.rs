use bevy::prelude::*;

use super::{game_entity::GameEntityType, vitality::DeathEvent};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScore::default()).add_systems(
            Update,
            (
                update_score_on_deaths,
                update_score_timer,
                // log_score.after(update_score_on_deaths),
            ),
        );
    }
}

fn update_score_timer(time: Res<Time>, mut game_score: ResMut<GameScore>) {
    game_score.tick(&time);
}

fn update_score_on_deaths(
    mut game_score: ResMut<GameScore>,
    mut death_events: EventReader<DeathEvent>,
) {
    for ev in death_events.iter() {
        match ev._type() {
            GameEntityType::Enemy => {
                game_score.add_score(10);
                game_score.increment_multiplier();
            }
            GameEntityType::Player => game_score.reset(),
            _ => (),
        }
    }
}

fn log_score(game_score: Res<GameScore>) {
    if game_score.is_changed() {
        println!("Score: {}", game_score.total());
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Score
////////////////////////////////////////////////////////////////////////////////

#[derive(Resource, Debug)]
pub struct GameScore {
    locked_in_score: u64,
    current_multiplier_score: u64,
    multiplier: u64,
    multiplier_timer: Option<Timer>,
    max_multiplier: u64,
}

impl Default for GameScore {
    fn default() -> Self {
        Self {
            locked_in_score: 0,
            current_multiplier_score: 0,
            multiplier: 0,
            multiplier_timer: None,
            max_multiplier: 20,
        }
    }
}

impl GameScore {
    pub fn total(&self) -> u64 {
        self.locked_in_score + self.current_multiplier_score
    }

    pub fn multiplier(&self) -> u64 {
        self.multiplier
    }

    pub fn locked_in_score(&self) -> u64 {
        self.locked_in_score
    }

    pub fn current_multiplier_score(&self) -> u64 {
        self.current_multiplier_score
    }

    pub fn add_score(&mut self, score: u64) {
        self.current_multiplier_score += score;
    }

    pub fn tick(&mut self, time: &Time) {
        if let Some(timer) = &mut self.multiplier_timer {
            if timer.tick(time.delta()).just_finished() {
                self.locked_in_score += self.current_multiplier_score * self.multiplier;
                self.current_multiplier_score = 0;
                self.multiplier_timer = None;
                self.multiplier = 1;
            }
        }
    }

    pub fn multiplier_time_percent_left(&self) -> Option<f32> {
        self.multiplier_timer.as_ref().map(|t| t.percent_left())
    }

    pub fn multiplier_timer(&self) -> &Option<Timer> {
        &self.multiplier_timer
    }

    pub fn add_multiplier(&mut self, multiplier: u64) {
        self.set_multiplier(self.multiplier + multiplier);
    }

    pub fn set_multiplier(&mut self, multiplier: u64) {
        self.multiplier = multiplier.min(self.max_multiplier);
        self.multiplier_timer = Some(create_timer());
    }

    pub fn max_multiplier(&self) -> u64 {
        self.max_multiplier
    }

    pub fn is_max_multiplier(&self) -> bool {
        self.multiplier == self.max_multiplier
    }

    pub fn increment_multiplier(&mut self) {
        self.set_multiplier(self.multiplier + 1);
    }

    pub fn reset(&mut self) {
        self.locked_in_score = 0;
        self.current_multiplier_score = 0;
        self.multiplier = 1;
        self.multiplier_timer = None;
    }
}

fn create_timer() -> Timer {
    Timer::from_seconds(10.0, TimerMode::Once)
}
