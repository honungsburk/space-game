#![allow(dead_code)]

use clap::Parser;
use space_game::file_save::FileSave;
use space_game::settings::Settings;

fn main() {
    // Parse Command Line Arguments
    let cli = space_game::cli::Cli::parse();

    let settings_path = cli.settings.as_deref().unwrap_or("settings.toml");

    // Load Settings
    let settings = Settings::load_from_file(settings_path).unwrap_or_else(|err| {
        eprintln!("Error loading settings.toml: {}", err);
        eprintln!("Using default settings.");
        Settings::default()
    });

    if cli.settings.is_none() {
        settings
            .save_to_file("settings.toml")
            .unwrap_or_else(|err| {
                eprintln!("Error saving settings.toml: {}", err);
            });
    }

    // Load High Scores
    let high_scores = space_game::game::score::HighScores::load_from_file("high_scores.toml")
        .unwrap_or_else(|err| {
            eprintln!("Error loading high_scores.toml: {}", err);
            eprintln!("Using default high scores.");
            space_game::game::score::HighScores::default()
        });

    space_game::run(cli.override_settings(&settings), high_scores);
}
