use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub humidity: i32,
    pub wind_speed: f64,
    pub description: String,
    pub icon: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastData {
    pub date: String,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirQualityData {
    pub aqi: i32,
    pub category: String,
    pub color: String,
    pub pm25: f64,
    pub pm10: f64,
    pub o3: f64,
    pub no2: f64,
    pub so2: f64,
    pub co: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub current: WeatherData,
    pub forecast: Vec<ForecastData>,
}

// Open-Meteo API Response Structures
#[derive(Debug, Serialize, Deserialize)]
struct OpenMeteoResponse {
    latitude: f64,
    longitude: f64,
    timezone: String,
    hourly: OpenMeteoHourly,
    hourly_units: OpenMeteoHourlyUnits,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenMeteoHourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    relative_humidity_2m: Vec<f64>,
    pressure_msl: Vec<f64>,
    wind_speed_10m: Vec<f64>,
    rain: Vec<f64>,
    showers: Vec<f64>,
    snowfall: Vec<f64>,
    dew_point_2m: Vec<f64>,
    cloud_cover: Vec<f64>,
    visibility: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenMeteoHourlyUnits {
    time: String,
    temperature_2m: String,
    relative_humidity_2m: String,
    pressure_msl: String,
    wind_speed_10m: String,
    rain: String,
    showers: String,
    snowfall: String,
    dew_point_2m: String,
    cloud_cover: String,
    visibility: String,
}

// AirVisual API structures removed - using mock data instead

fn get_aqi_category(aqi: i32) -> (String, String) {
    match aqi {
        0..=50 => ("Good".to_string(), "#00E400".to_string()),
        51..=100 => ("Moderate".to_string(), "#FFFF00".to_string()),
        101..=150 => ("Unhealthy for Sensitive Groups".to_string(), "#FF7E00".to_string()),
        151..=200 => ("Unhealthy".to_string(), "#FF0000".to_string()),
        201..=300 => ("Very Unhealthy".to_string(), "#8F3F97".to_string()),
        _ => ("Hazardous".to_string(), "#7E0023".to_string()),
    }
}

fn get_weather_description(cloud_cover: f64, rain: f64, showers: f64, snowfall: f64) -> (String, String) {
    if snowfall > 0.0 {
        ("Snow".to_string(), "❄️".to_string())
    } else if rain > 0.0 || showers > 0.0 {
        ("Rain".to_string(), "🌧️".to_string())
    } else if cloud_cover > 80.0 {
        ("Overcast".to_string(), "☁️".to_string())
    } else if cloud_cover > 50.0 {
        ("Partly Cloudy".to_string(), "⛅".to_string())
    } else if cloud_cover > 20.0 {
        ("Mostly Clear".to_string(), "🌤️".to_string())
    } else {
        ("Clear".to_string(), "☀️".to_string())
    }
}

// Real API Functions
#[tauri::command]
async fn get_weather(latitude: f64, longitude: f64) -> Result<WeatherResponse, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,relative_humidity_2m,pressure_msl,wind_speed_10m,rain,showers,snowfall,dew_point_2m,cloud_cover,visibility",
        latitude, longitude
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch weather data: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Weather API error: {}", response.status()));
    }

    let weather_data: OpenMeteoResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse weather data: {}", e))?;

    // Get current weather (first hour of data)
    let current_temp = weather_data.hourly.temperature_2m[0];
    let current_humidity = weather_data.hourly.relative_humidity_2m[0] as i32;
    let current_wind_speed = weather_data.hourly.wind_speed_10m[0];
    let current_cloud_cover = weather_data.hourly.cloud_cover[0];
    let current_rain = weather_data.hourly.rain[0];
    let current_showers = weather_data.hourly.showers[0];
    let current_snowfall = weather_data.hourly.snowfall[0];

    // Determine weather description based on conditions
    let (description, icon) = get_weather_description(current_cloud_cover, current_rain, current_showers, current_snowfall);

    // Process forecast data - get next 3 days from hourly data
    let mut forecast = Vec::new();
    let mut processed_dates = std::collections::HashSet::new();
    let mut day_count = 0;

    for (i, time_str) in weather_data.hourly.time.iter().enumerate() {
        if day_count >= 3 {
            break;
        }

        // Extract date from ISO string (e.g., "2025-09-17T00:00" -> "2025-09-17")
        let date = time_str.split('T').next().unwrap_or("");
        
        if !processed_dates.contains(date) && i > 0 { // Skip first day (today)
            processed_dates.insert(date.to_string());
            day_count += 1;

            // Find all hours for this date
            let mut day_temps = Vec::new();
            let mut day_cloud_cover = Vec::new();
            let mut day_rain = Vec::new();
            let mut day_showers = Vec::new();
            let mut day_snowfall = Vec::new();

            for (j, other_time) in weather_data.hourly.time.iter().enumerate() {
                let other_date = other_time.split('T').next().unwrap_or("");
                if other_date == date {
                    day_temps.push(weather_data.hourly.temperature_2m[j]);
                    day_cloud_cover.push(weather_data.hourly.cloud_cover[j]);
                    day_rain.push(weather_data.hourly.rain[j]);
                    day_showers.push(weather_data.hourly.showers[j]);
                    day_snowfall.push(weather_data.hourly.snowfall[j]);
                }
            }

            if !day_temps.is_empty() {
                let max_temp = day_temps.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let min_temp = day_temps.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let avg_cloud_cover = day_cloud_cover.iter().sum::<f64>() / day_cloud_cover.len() as f64;
                let total_rain = day_rain.iter().sum::<f64>();
                let total_showers = day_showers.iter().sum::<f64>();
                let total_snowfall = day_snowfall.iter().sum::<f64>();

                let (day_description, day_icon) = get_weather_description(avg_cloud_cover, total_rain, total_showers, total_snowfall);

                let day_name = match day_count {
                    1 => "Tomorrow",
                    2 => "Day After",
                    3 => "3 Days",
                    _ => "Unknown",
                };

                forecast.push(ForecastData {
                    date: day_name.to_string(),
                    temperature_max: max_temp,
                    temperature_min: min_temp,
                    description: day_description,
                    icon: day_icon,
                });
            }
        }
    }

    // Create current weather data
    let current = WeatherData {
        temperature: current_temp,
        humidity: current_humidity,
        wind_speed: current_wind_speed,
        description,
        icon,
        city: format!("{:.2}°N, {:.2}°E", latitude, longitude),
        country: weather_data.timezone,
    };

    Ok(WeatherResponse { current, forecast })
}

#[tauri::command]
async fn get_air_quality(_latitude: f64, _longitude: f64) -> Result<AirQualityData, String> {
    // For now, we'll use mock data since AirVisual API requires a key
    // In the future, you could integrate with other free air quality APIs
    get_mock_air_quality(_latitude, _longitude).await
}


#[tauri::command]
async fn get_mock_weather(latitude: f64, longitude: f64) -> Result<WeatherResponse, String> {
    // Mock data for demo purposes
    let current = WeatherData {
        temperature: 22.5,
        humidity: 65,
        wind_speed: 3.2,
        description: "Partly cloudy".to_string(),
        icon: "⛅".to_string(),
        city: format!("{:.2}°N, {:.2}°E", latitude, longitude),
        country: "GMT".to_string(),
    };

    let forecast = vec![
        ForecastData {
            date: "Tomorrow".to_string(),
            temperature_max: 25.0,
            temperature_min: 18.0,
            description: "Sunny".to_string(),
            icon: "☀️".to_string(),
        },
        ForecastData {
            date: "Day After".to_string(),
            temperature_max: 23.0,
            temperature_min: 16.0,
            description: "Partly cloudy".to_string(),
            icon: "⛅".to_string(),
        },
        ForecastData {
            date: "3 Days".to_string(),
            temperature_max: 20.0,
            temperature_min: 14.0,
            description: "Light rain".to_string(),
            icon: "🌧️".to_string(),
        },
    ];

    Ok(WeatherResponse { current, forecast })
}

#[tauri::command]
async fn get_mock_air_quality(_latitude: f64, _longitude: f64) -> Result<AirQualityData, String> {
    // Mock data for demo purposes
    let aqi = 45;
    let (category, color) = get_aqi_category(aqi);

    Ok(AirQualityData {
        aqi,
        category,
        color,
        pm25: 12.5,
        pm10: 20.0,
        o3: 0.04,
        no2: 0.015,
        so2: 0.008,
        co: 0.3,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_weather,
            get_air_quality,
            get_mock_weather,
            get_mock_air_quality
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
