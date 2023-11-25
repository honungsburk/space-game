use clap::{Parser, ValueEnum};

use crate::{
    game::debug::VisualDebug,
    settings::{ResolutionSetting, Settings},
};

/// A Space Game
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show visual debug information.
    /// Takes a list of flags.
    /// CLI flags are joined with the settings file flags.
    ///
    /// Example: `--visual-debug background-grid camera-position`
    #[arg(long, value_enum, num_args = 1..)]
    pub visual_debug: Vec<VisualDebug>, // NOTE: HashSet<VisualDebug> is not supported by clap

    /// Path to settings file.
    /// If not specified, the default settings will be used
    #[arg(long)]
    pub settings: Option<String>,

    /// Set the x resolution.
    #[arg(long, short)]
    pub x_pixels: Option<u32>,

    /// Set the y resolution.
    #[arg(long, short)]
    pub y_pixels: Option<u32>,

    /// Sets a bunch of settings to make the game look good on social media.
    /// Overrides the x and y resolution settings.
    #[arg(long, value_enum)]
    pub social: Option<SocialMediaFormat>,
}

impl Cli {
    pub fn override_settings(&self, settings: &Settings) -> Settings {
        let mut new_config = (*settings).clone();

        if let Some(x) = self.x_pixels {
            new_config.window.resolution.x = x;
        }

        if let Some(y) = self.y_pixels {
            new_config.window.resolution.y = y;
        }

        if let Some(social) = self.social {
            new_config.window.resolution = social.resolution();
        }

        for debug in self.visual_debug.iter() {
            new_config.visual_debug.insert(*debug);
        }

        new_config
    }
}

/// Optimize the game for a specific social media platform
///
/// For now, it only means changing the resolution to match the platform's
/// recommended resolution.
///
/// TODO: there is a problem where that putting a video in 9:16 aspect ratio
/// doesn't work on the Mac Retina display. It should have enough pixels but
/// it ends up with another resolution instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum SocialMediaFormat {
    Instagram,
    FullHD,
    Short,
}

impl SocialMediaFormat {
    pub fn resolution(self) -> ResolutionSetting {
        match self {
            SocialMediaFormat::Instagram => ResolutionSetting { x: 1080, y: 1350 },
            SocialMediaFormat::FullHD => ResolutionSetting { x: 1920, y: 1080 },
            SocialMediaFormat::Short => ResolutionSetting { x: 720, y: 1280 },
        }
    }
}
