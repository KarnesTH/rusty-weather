use dotenv::{dotenv, var};
use lingua_i18n_rs::prelude::Lingua;
use serde::{Deserialize, Serialize};

const PRINT_WIDTH: usize = 70;

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub location: WeatherLocation,
    pub current: WeatherCurrent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherLocation {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherCurrent {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i64,
    pub condition: WeatherCondition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i64,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

enum LineType {
    Simple,
    Double,
}

impl Default for Weather {
    fn default() -> Self {
        Self::new()
    }
}

impl Weather {
    /// Create a new Weather instance
    pub fn new() -> Self {
        Weather {
            location: WeatherLocation {
                name: "".to_string(),
                region: "".to_string(),
                country: "".to_string(),
                lat: 0.0,
                lon: 0.0,
                tz_id: "".to_string(),
                localtime_epoch: 0,
                localtime: "".to_string(),
            },
            current: WeatherCurrent {
                last_updated_epoch: 0,
                last_updated: "".to_string(),
                temp_c: 0.0,
                temp_f: 0.0,
                is_day: 0,
                condition: WeatherCondition {
                    text: "".to_string(),
                    icon: "".to_string(),
                    code: 0,
                },
                wind_mph: 0.0,
                wind_kph: 0.0,
                wind_degree: 0,
                wind_dir: "".to_string(),
                pressure_mb: 0.0,
                pressure_in: 0.0,
                precip_mm: 0.0,
                precip_in: 0.0,
                humidity: 0,
                cloud: 0,
                feelslike_c: 0.0,
                feelslike_f: 0.0,
                vis_km: 0.0,
                vis_miles: 0.0,
                uv: 0.0,
                gust_mph: 0.0,
                gust_kph: 0.0,
            },
        }
    }

    /// Fetch the current weather for a given city
    ///
    /// # Arguments
    ///
    /// * `city` - A string slice that holds the name of the city
    ///
    /// # Returns
    ///
    /// A Result object with the Weather struct or an error
    ///
    /// # Errors
    ///
    /// If the request fails or the response cannot be deserialized
    pub async fn get_current_weather(
        &self,
        city: String,
        lang: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let url = &format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no&lang={}",
            var("WEATHER_API").unwrap(),
            city,
            lang
        );

        let resp = reqwest::get(url).await;
        let data = serde_json::from_str(&resp?.text().await?)?;

        Ok(data)
    }

    /// Print the current weather information
    ///
    /// This method prints the current weather information in a formatted way
    /// to the console
    pub fn print_current_weather(&self) {
        let width = PRINT_WIDTH;

        self.print_separator(width, 't');
        self.print_line(
            "current.weather_report",
            &[("city", self.location.name.as_str())],
            width,
            LineType::Simple,
        );
        self.print_separator(width, 'm');
        self.print_line(
            "current.date",
            &[("date", self.location.localtime.as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.region",
            &[
                ("region", self.location.region.as_str()),
                ("country", self.location.country.as_str()),
            ],
            width,
            LineType::Double,
        );
        self.print_separator(width, 'm');
        self.print_line("current.current_conditions", &[], width, LineType::Simple);
        self.print_separator(width, 'm');
        self.print_line(
            "current.status",
            &[("status", self.current.condition.text.as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.temperature",
            &[
                ("temperature_c", self.current.temp_c.to_string().as_str()),
                ("temperature_f", self.current.temp_f.to_string().as_str()),
            ],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.feels_like",
            &[
                (
                    "feels_like_c",
                    self.current.feelslike_c.to_string().as_str(),
                ),
                (
                    "feels_like_f",
                    self.current.feelslike_f.to_string().as_str(),
                ),
            ],
            width,
            LineType::Double,
        );
        // TODO: Create a better way to handle the daytime
        self.print_line(
            "current.daytime",
            &[(
                "daytime",
                if self.current.is_day == 1 {
                    "day"
                } else {
                    "night"
                },
            )],
            width,
            LineType::Double,
        );

        self.print_separator(width, 'm');
        self.print_line("current.more_details", &[], width, LineType::Simple);
        self.print_separator(width, 'm');
        self.print_line(
            "current.humidity",
            &[("humidity", self.current.humidity.to_string().as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.cloudiness",
            &[("cloudiness", self.current.cloud.to_string().as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.wind",
            &[
                ("wind_speed", self.current.wind_kph.to_string().as_str()),
                ("wind_direction", self.current.wind_dir.as_str()),
            ],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.gusts",
            &[
                ("wind_gusts", self.current.gust_kph.to_string().as_str()),
                ("gusts_mph", self.current.gust_mph.to_string().as_str()),
            ],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.precipitation",
            &[("precipitation", self.current.precip_mm.to_string().as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.pressure",
            &[("pressure", self.current.pressure_mb.to_string().as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.visibility",
            &[("visibility", self.current.vis_km.to_string().as_str())],
            width,
            LineType::Double,
        );
        self.print_line(
            "current.uv_index",
            &[("uv_index", self.current.uv.to_string().as_str())],
            width,
            LineType::Double,
        );

        self.print_separator(width, 'm');
        self.print_line(
            "current.last_updated",
            &[("last_updated", self.current.last_updated.as_str())],
            width,
            LineType::Double,
        );
        self.print_separator(width, 'b');
    }

    /// Helper method to print a line
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key for the translation
    /// * `content` - A slice of tuples that holds the content for the translation
    fn print_line(&self, key: &str, content: &[(&str, &str)], width: usize, line_type: LineType) {
        let formatted = match line_type {
            LineType::Simple => format!("│ {}", Lingua::t(key, content).unwrap().to_uppercase()),
            LineType::Double => format!("│ {}", Lingua::t(key, content).unwrap()),
        };
        let padding = width - formatted.chars().count();
        println!("{}{} │", formatted, " ".repeat(padding));
    }

    /// Print a separator line
    ///
    /// # Arguments
    ///
    /// * `width` - An integer that holds the width of the separator
    /// * `style` - A char that holds the style of the separator
    fn print_separator(&self, width: usize, style: char) {
        println!(
            "{}{}{}",
            match style {
                't' => "┌",
                'b' => "└",
                'm' => "├",
                _ => "├",
            },
            "─".repeat(width),
            match style {
                't' => "┐",
                'b' => "┘",
                'm' => "┤",
                _ => "┤",
            }
        );
    }
}
