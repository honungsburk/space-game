pub mod game_score;
pub mod high_score;

use bevy::prelude::*;

use crate::file_save::FileSave;

// re-export
pub use self::game_score::*;
pub use self::high_score::*;

use super::events::GameOverEvent;
use super::events::HighScoreEvent;
use super::{game_entity::GameEntityType, vitality::DeathEvent};

pub struct ScorePlugin {
    pub high_scores: HighScores,
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScore::default())
            .insert_resource(self.high_scores.clone())
            .add_event::<HighScoreEvent>()
            .add_systems(
                Update,
                (
                    update_score_on_deaths,
                    update_score_timer,
                    update_high_scores_on_game_over,
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

fn update_high_scores_on_game_over(
    mut high_scores: ResMut<HighScores>,
    mut game_over_events: EventReader<GameOverEvent>,
    mut high_score_events: EventWriter<HighScoreEvent>,
) {
    for GameOverEvent { score } in game_over_events.iter() {
        let player_name = String::from("Player");
        if let Some(placement) = high_scores.add_name_score(player_name.clone(), *score) {
            high_score_events.send(HighScoreEvent {
                player_name,
                placement,
                score: *score,
            });

            // Need some way of telling the user that the game is saving...
            // How to prevent corrupted data?
            let _ = high_scores.save_to_file("high_scores.toml");
        }
    }
}
