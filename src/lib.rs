mod utils;
mod weather;

pub mod prelude {
    pub use crate::utils::*;
    pub use crate::weather::Weather;
    pub use crate::{Cli, WeatherCommand};
}

use clap::Parser;

/// A CLI weather app to get weather information for a given city
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: WeatherCommand,
}

#[derive(Parser)]
pub enum WeatherCommand {
    /// Get the current weather for a city
    Current {
        /// The city to get the weather for
        #[clap(short, long)]
        city: String,
    },
    /// Configure the language for the weather app
    Language {
        #[clap(subcommand)]
        commands: LanguageCommand,
    },
}

#[derive(Parser)]
pub enum LanguageCommand {
    /// Set the language for the weather app
    Set {
        /// The language to set
        #[clap(short, long)]
        lang: String,
    },
    /// List the available languages
    List,
    /// Get the current language
    Get,
}
