mod utils;
mod weather;

pub mod prelude {
    pub use crate::utils::*;
    pub use crate::weather::{ForecastWeather, Weather};
    pub use crate::{Cli, LanguageCommand, WeatherCommand};
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
    /// Get the forcast weather for a city and days
    Forecast {
        /// The city to get the weather for
        #[clap(short, long)]
        city: String,
        /// The days to get the weather for
        #[clap(short, long)]
        days: usize,
    },
    /// Configure the language for the weather app
    Language {
        #[clap(subcommand)]
        commands: LanguageCommand,
    },
    /// Start a web server to serve the weather app
    Web {
        /// The port to start the web server on
        #[clap(short, long)]
        port: u16,
    },
    /// Start a api server to serve the weather app
    Serve,
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
