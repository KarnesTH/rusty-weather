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
pub struct ForecastWeather {
    pub location: WeatherLocation,
    pub forecast: WeatherForecast,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastDay {
    pub date: String,
    pub day: Day,
    pub astro: Astro,
    pub hour: Vec<Hour>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub maxtemp_c: f64,
    pub maxtemp_f: f64,
    pub mintemp_c: f64,
    pub mintemp_f: f64,
    pub avgtemp_c: f64,
    pub avgtemp_f: f64,
    pub maxwind_mph: f64,
    pub maxwind_kph: f64,
    pub totalprecip_mm: f64,
    pub totalprecip_in: f64,
    pub totalsnow_cm: f64,
    pub avgvis_km: f64,
    pub avgvis_miles: f64,
    pub avghumidity: i64,
    pub daily_will_it_rain: i64,
    pub daily_chance_of_rain: i64,
    pub daily_will_it_snow: i64,
    pub daily_chance_of_snow: i64,
    pub condition: WeatherCondition,
    pub uv: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moon_phase: String,
    pub moon_illumination: f64,
    pub is_moon_up: i64,
    pub is_sun_up: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hour {
    pub time: String,
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
    pub snow_cm: f64,
    pub humidity: i64,
    pub cloud: i64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub windchill_c: f64,
    pub windchill_f: f64,
    pub heatindex_c: f64,
    pub heatindex_f: f64,
    pub dewpoint_c: f64,
    pub dewpoint_f: f64,
    pub will_it_rain: i64,
    pub chance_of_rain: i64,
    pub will_it_snow: i64,
    pub chance_of_snow: i64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
    pub uv: f64,
}

enum LineType {
    Header,
    Text,
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

        print_separator(width, 't');
        print_line(
            "current.weather_report",
            &[("city", self.location.name.as_str())],
            width,
            LineType::Header,
        );
        print_separator(width, 'm');
        print_line(
            "current.date",
            &[("date", self.location.localtime.as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.region",
            &[
                ("region", self.location.region.as_str()),
                ("country", self.location.country.as_str()),
            ],
            width,
            LineType::Text,
        );
        print_separator(width, 'm');
        print_line("current.current_conditions", &[], width, LineType::Header);
        print_separator(width, 'm');
        print_line(
            "current.status",
            &[("status", self.current.condition.text.as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.temperature",
            &[
                ("temperature_c", self.current.temp_c.to_string().as_str()),
                ("temperature_f", self.current.temp_f.to_string().as_str()),
            ],
            width,
            LineType::Text,
        );
        print_line(
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
            LineType::Text,
        );
        // TODO: Create a better way to handle the daytime
        print_line(
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
            LineType::Text,
        );

        print_separator(width, 'm');
        print_line("current.more_details", &[], width, LineType::Header);
        print_separator(width, 'm');
        print_line(
            "current.humidity",
            &[("humidity", self.current.humidity.to_string().as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.cloudiness",
            &[("cloudiness", self.current.cloud.to_string().as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.wind",
            &[
                ("wind_speed", self.current.wind_kph.to_string().as_str()),
                ("wind_direction", self.current.wind_dir.as_str()),
            ],
            width,
            LineType::Text,
        );
        print_line(
            "current.gusts",
            &[
                ("wind_gusts", self.current.gust_kph.to_string().as_str()),
                ("gusts_mph", self.current.gust_mph.to_string().as_str()),
            ],
            width,
            LineType::Text,
        );
        print_line(
            "current.precipitation",
            &[("precipitation", self.current.precip_mm.to_string().as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.pressure",
            &[("pressure", self.current.pressure_mb.to_string().as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.visibility",
            &[("visibility", self.current.vis_km.to_string().as_str())],
            width,
            LineType::Text,
        );
        print_line(
            "current.uv_index",
            &[("uv_index", self.current.uv.to_string().as_str())],
            width,
            LineType::Text,
        );

        print_separator(width, 'm');
        print_line(
            "current.last_updated",
            &[("last_updated", self.current.last_updated.as_str())],
            width,
            LineType::Text,
        );
        print_separator(width, 'b');
    }
}

impl Default for ForecastWeather {
    fn default() -> Self {
        Self::new()
    }
}

impl ForecastWeather {
    pub fn new() -> Self {
        ForecastWeather {
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
            forecast: WeatherForecast {
                forecastday: Vec::new(),
            },
        }
    }

    pub async fn get_forecast_weather(
        &self,
        days: usize,
        city: String,
        lang: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let url = &format!(
            "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&aqi=no&lang={}&days={}&alerts=no",
            var("WEATHER_API").unwrap(),
            city,
            lang,
            days
        );

        let resp = reqwest::get(url).await;
        let data = serde_json::from_str(&resp?.text().await?)?;

        Ok(data)
    }

    pub fn print_forecast_weather(&self) {
        let width = PRINT_WIDTH;
        print_separator(width, 't');
        print_line(
            "forecast.forecast",
            &[("city", self.location.name.as_str())],
            width,
            LineType::Header,
        );

        for forecast_day in &self.forecast.forecastday {
            print_separator(width, 'm');
            print_line(
                "forecast.day_forecast",
                &[("date", forecast_day.date.as_str())],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.temperature_range",
                &[
                    (
                        "min_temp_c",
                        forecast_day.day.mintemp_c.to_string().as_str(),
                    ),
                    (
                        "min_temp_f",
                        forecast_day.day.mintemp_f.to_string().as_str(),
                    ),
                    (
                        "max_temp_c",
                        forecast_day.day.maxtemp_c.to_string().as_str(),
                    ),
                    (
                        "max_temp_f",
                        forecast_day.day.maxtemp_f.to_string().as_str(),
                    ),
                ],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.avg_temperature",
                &[
                    (
                        "avg_temp_c",
                        forecast_day.day.avgtemp_c.to_string().as_str(),
                    ),
                    (
                        "avg_temp_f",
                        forecast_day.day.avgtemp_f.to_string().as_str(),
                    ),
                ],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.condition",
                &[("condition", forecast_day.day.condition.text.as_str())],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.precipitation",
                &[(
                    "precipitation",
                    forecast_day.day.totalprecip_mm.to_string().as_str(),
                )],
                width,
                LineType::Text,
            );
            if forecast_day.day.daily_will_it_rain > 0 {
                print_line(
                    "forecast.chance_of_rain",
                    &[(
                        "rain_chance",
                        forecast_day.day.daily_chance_of_rain.to_string().as_str(),
                    )],
                    width,
                    LineType::Text,
                );
            }
            if forecast_day.day.daily_will_it_snow > 0 {
                print_line(
                    "forecast.chance_of_snow",
                    &[(
                        "snow_chance",
                        forecast_day.day.daily_chance_of_snow.to_string().as_str(),
                    )],
                    width,
                    LineType::Text,
                );
            }
            print_line(
                "forecast.wind",
                &[
                    (
                        "wind_kph",
                        forecast_day.day.maxwind_kph.to_string().as_str(),
                    ),
                    (
                        "wind_mph",
                        forecast_day.day.maxwind_mph.to_string().as_str(),
                    ),
                ],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.humidity",
                &[(
                    "humidity",
                    forecast_day.day.avghumidity.to_string().as_str(),
                )],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.uv_index",
                &[("uv", forecast_day.day.uv.to_string().as_str())],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.sunrise",
                &[("sunrise", forecast_day.astro.sunrise.as_str())],
                width,
                LineType::Text,
            );
            print_line(
                "forecast.sunset",
                &[("sunset", forecast_day.astro.sunset.as_str())],
                width,
                LineType::Text,
            );
        }

        print_separator(width, 'b');
    }
}

/// Helper function to print a line
///
/// # Arguments
///
/// * `key` - A string slice that holds the key for the translation
/// * `content` - A slice of tuples that holds the content for the translation
/// * `width` - A width to calculate the padding
/// * `line_type` - A line type for the output
fn print_line(key: &str, content: &[(&str, &str)], width: usize, line_type: LineType) {
    let text = match line_type {
        LineType::Header => Lingua::t(key, content).unwrap().to_uppercase(),
        LineType::Text => Lingua::t(key, content).unwrap(),
    };
    let formatted = format!("| {}", text);
    let padding = width - formatted.chars().count();
    println!("{}{} │", formatted, " ".repeat(padding));
}

/// Helper function to print a separator line
///
/// # Arguments
///
/// * `width` - An integer that holds the width of the separator
/// * `style` - A char that holds the style of the separator
fn print_separator(width: usize, style: char) {
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
