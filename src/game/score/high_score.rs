use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, error::Error};
use toml::from_str;

use crate::file_save::{self, FileSave};

const MAX_NUMBER_OF_HIGH_SCORES: usize = 10;

#[derive(Deserialize, Serialize, Resource, Debug, PartialEq, Eq, Clone)]
pub struct HighScores {
    scores: Vec<HighScore>,
}

impl HighScores {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    /// Returns the index of the score if it was added, or None if it was not added.
    pub fn add_score(&mut self, score: HighScore) -> Option<usize> {
        let index = self.scores.iter().position(|s| s < &score);
        if let Some(index) = index {
            self.scores.insert(index, score);
            self.scores.truncate(MAX_NUMBER_OF_HIGH_SCORES);
            Some(index)
        } else if self.len() < MAX_NUMBER_OF_HIGH_SCORES {
            self.scores.push(score);
            Some(self.len() - 1)
        } else {
            None
        }
    }

    pub fn add_name_score(&mut self, name: String, score: u64) -> Option<usize> {
        self.add_score(HighScore {
            player_name: name,
            score,
        })
    }

    pub fn sort(&mut self) {
        self.scores.sort();
    }

    pub fn len(&self) -> usize {
        self.scores.len()
    }
}

impl FileSave for HighScores {
    type Item = HighScores;
    fn load_from_file(path: &str) -> Result<HighScores, Box<dyn Error>> {
        let contents = file_save::load_from_file(path)?;
        let mut config: HighScores = from_str(&contents)?;
        config.sort();
        Ok(config)
    }

    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string(&self)?;
        file_save::save_to_file(path, &contents)?;
        Ok(())
    }
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct HighScore {
    pub player_name: String,
    pub score: u64,
}

impl Ord for HighScore {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.score, &self.player_name).cmp(&(other.score, &other.player_name))
    }
}

impl PartialOrd for HighScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
