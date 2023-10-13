use clap::Parser;

use crate::config::{Config, LogLevel, Scene, VisualDebug};

/// A Space Game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// How detailed the logs should be
    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,

    /// What file to write logs to
    /// If not specified, logs will be written to stdout
    #[arg(long)]
    pub log_file: Option<String>,

    /// Show visual debug information
    /// Takes a list of options
    /// If not specified, no visual debug information will be shown
    #[arg(long, value_enum)]
    pub visual_debug: Vec<VisualDebug>, // NOTE: HashSet<VisualDebug> is not supported by clap

    /// Path to config file
    /// If not specified, the default config will be used
    #[arg(long)]
    pub config: Option<String>,

    /// Path to settings file
    /// If not specified, the default settings will be used
    #[arg(long)]
    pub settings: Option<String>,

    /// Scene to load
    /// If not specified, the default scene will be used
    #[arg(long, value_enum)]
    pub scene: Option<Scene>,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            log_level: None,
            log_file: None,
            visual_debug: vec![],
            config: None,
            settings: None,
            scene: None,
        }
    }
}

impl Cli {
    pub fn override_config(&self, config: &Config) -> Config {
        let mut new_config = (*config).clone();

        if let Some(log_level) = self.log_level {
            new_config.log_level = log_level;
        }

        if let Some(log_file) = &self.log_file {
            new_config.log_file = Some(log_file.clone());
        }

        if !self.visual_debug.is_empty() {
            new_config.visual_debug = self.visual_debug.iter().cloned().collect();
        }

        if let Some(scene) = self.scene {
            new_config.scene = scene;
        }

        new_config
    }
}
