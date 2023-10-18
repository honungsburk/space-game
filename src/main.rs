#![allow(dead_code)]

use clap::Parser;
use space_game::config::Config;
use space_game::file_save::FileSave;
use space_game::settings::Settings;

fn main() {
    // Parse Command Line Arguments
    let cli = space_game::cli::Cli::parse();

    let config_path = cli.config.as_deref().unwrap_or("config.toml");

    // Load Config
    let config = Config::load_from_file(config_path).unwrap_or_else(|err| {
        eprintln!("Error loading config.toml: {}", err);
        eprintln!("Using default config.");
        Config::default()
    });

    if cli.config.is_none() {
        config.save_to_file(&config_path).unwrap_or_else(|err| {
            eprintln!("Error saving config.toml: {}", err);
        });
    }

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
    let mut high_scores = space_game::game::score::HighScores::load_from_file("high_scores.toml")
        .unwrap_or_else(|err| {
            eprintln!("Error loading high_scores.toml: {}", err);
            eprintln!("Using default high scores.");
            space_game::game::score::HighScores::default()
        });

    space_game::run(cli.override_config(&config), settings, high_scores);
}
