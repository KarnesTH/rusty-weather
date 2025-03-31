use clap::Parser;
use rusty_weather::prelude::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.commands {
        WeatherCommand::Current { city } => {
            let weather = Weather::new();
            let result = weather.fetch_current_weather(city).await;
            match result {
                Ok(weather) => {
                    weather.print_current_weather();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
