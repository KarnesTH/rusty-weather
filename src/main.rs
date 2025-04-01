use clap::Parser;
use rusty_weather::prelude::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.commands {
        WeatherCommand::Current { city, lang } => {
            let weather = Weather::new();
            let result = weather.get_current_weather(city, lang).await;
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
