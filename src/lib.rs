mod utils;
mod weather;

pub mod prelude {
    pub use crate::utils::*;
    pub use crate::weather::{ForecastWeather, Weather};
    pub use crate::{Cli, LanguageCommand, WeatherCommand};
}

use clap::Parser;

#[derive(Parser)]
#[clap(
    version,
    about = "A CLI weather app to get weather information for a given city",
    author = "KarnesTH"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: WeatherCommand,
}

#[derive(Parser)]
pub enum WeatherCommand {
    #[clap(about = "Get the current weather for a city")]
    Current {
        #[clap(short, long, help = "The city to get the weather for")]
        city: String,
    },
    #[clap(about = "Get the forcast weather for a city and days")]
    Forecast {
        #[clap(short, long, help = "The city to get the weather for")]
        city: String,
        #[clap(short, long, help = "The days to get the weather for")]
        days: usize,
    },
    #[clap(about = "Configure the language for the weather app")]
    Language {
        #[clap(subcommand)]
        commands: LanguageCommand,
    }
}

#[derive(Parser)]
pub enum LanguageCommand {
    #[clap(about = "Set the language for the weather app")]
    Set {
        #[clap(short, long, help = "The language to set")]
        lang: String,
    },
    #[clap(about = "List the available languages")]
    List,
    #[clap(about = "Get the current language")]
    Get,
}
