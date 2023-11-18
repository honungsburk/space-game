use clap::Parser;

use crate::settings::Settings;

/// A Space Game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show visual debug information
    /// Takes a list of options
    /// If not specified, no visual debug information will be shown
    // #[arg(long, value_enum)]
    // pub visual_debug: Vec<VisualDebug>, // NOTE: HashSet<VisualDebug> is not supported by clap

    /// Path to settings file
    /// If not specified, the default settings will be used
    #[arg(long)]
    pub settings: Option<String>,

    /// Set the x resolution
    #[arg(long, short)]
    pub x_pixels: Option<u32>,

    /// Set the y resolution
    #[arg(long, short)]
    pub y_pixels: Option<u32>,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            settings: None,
            x_pixels: None,
            y_pixels: None,
        }
    }
}

impl Cli {
    pub fn override_settings(&self, settings: &Settings) -> Settings {
        let mut new_config = (*settings).clone();

        // if !self.visual_debug.is_empty() {
        //     new_config.visual_debug = self.visual_debug.iter().cloned().collect();
        // }

        if let Some(x) = self.x_pixels {
            new_config.window.resolution.x = x;
        }

        if let Some(y) = self.y_pixels {
            new_config.window.resolution.y = y;
        }

        new_config
    }
}
