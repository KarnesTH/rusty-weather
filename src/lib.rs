use clap::Parser;

/// A CLI weather app to get weather information for a given city
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Parser)]
pub enum Command {
    /// Get the current weather for a city
    Current {
        /// The city to get the weather for
        #[clap(short, long)]
        city: String,
    },
}
