use dotenv::{dotenv, var};
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
        lang: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let url = &format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no&lang=d{}",
            var("WEATHER_API").unwrap(),
            city,
            lang.unwrap_or("de".to_string())
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
            None,
            format!("WETTERBERICHT FÜR {}", self.location.name.to_uppercase()),
            width,
            LineType::Simple,
        );
        self.print_separator(width, 'm');
        self.print_line(
            Some("Datum & Zeit"),
            self.location.localtime.clone(),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Region"),
            format!("{}, {}", self.location.region, self.location.country),
            width,
            LineType::Double,
        );
        self.print_separator(width, 'm');
        self.print_line(
            None,
            "AKTUELLE BEDINGUNGEN".to_string(),
            width,
            LineType::Simple,
        );
        self.print_separator(width, 'm');
        self.print_line(
            Some("Status"),
            self.current.condition.text.clone(),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Temperatur"),
            format!(
                "{:.1}°C ({:.1}°F)",
                self.current.temp_c, self.current.temp_f
            ),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Gefühlt wie"),
            format!(
                "{:.1}°C ({:.1}°F)",
                self.current.feelslike_c, self.current.feelslike_f
            ),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Tageszeit"),
            if self.current.is_day == 1 {
                "Tag"
            } else {
                "Nacht"
            }
            .to_string(),
            width,
            LineType::Double,
        );

        self.print_separator(width, 'm');
        self.print_line(None, "WEITERE DETAILS".to_string(), width, LineType::Simple);
        self.print_separator(width, 'm');
        self.print_line(
            Some("Luftfeutigkeit"),
            format!("{}%", self.current.humidity),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Bewölkung"),
            format!("{}%", self.current.cloud),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Wind"),
            format!(
                "Wind: {:.1} km/h (Richtung: {})",
                self.current.wind_kph, self.current.wind_dir
            ),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Windböen"),
            format!(
                "{:.1} km/h ({:.1} mph)",
                self.current.gust_kph, self.current.gust_mph
            ),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Niederschlag"),
            format!("{:.1} mm", self.current.precip_mm),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Luftdruck"),
            format!("{:.1} mb", self.current.pressure_mb),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("Sichtweite"),
            format!("{:.1} km", self.current.vis_km),
            width,
            LineType::Double,
        );
        self.print_line(
            Some("UV-Index"),
            format!("{:.1}", self.current.uv),
            width,
            LineType::Double,
        );

        self.print_separator(width, 'm');
        self.print_line(
            Some("Letzte Aktualisierung"),
            self.current.last_updated.clone(),
            width,
            LineType::Double,
        );
        self.print_separator(width, 'b');
    }

    /// Print a line with a label and content
    ///
    /// # Arguments
    ///
    /// * `label` - An optional string slice that holds the label for the content
    /// * `content` - A string slice that holds the content to print
    /// * `width` - An integer that holds the width of the line
    /// * `line_type` - A LineType enum that holds the type of line to print
    fn print_line(&self, label: Option<&str>, content: String, width: usize, line_type: LineType) {
        let formatted = match line_type {
            LineType::Simple => format!("│ {}", content),
            LineType::Double => format!("│ {}: {}", label.unwrap(), content),
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
