mod weather;

pub mod prelude {
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
        /// The language to get the weather in
        #[clap(short, long)]
        lang: Option<String>,
    },
}
