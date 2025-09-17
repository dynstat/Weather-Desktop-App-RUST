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

#[tauri::command]
async fn get_mock_weather(city: String) -> Result<WeatherResponse, String> {
    // Mock data for demo purposes
    let current = WeatherData {
        temperature: 22.5,
        humidity: 65,
        wind_speed: 3.2,
        description: "Partly cloudy".to_string(),
        icon: "02d".to_string(),
        city: city.clone(),
        country: "US".to_string(),
    };

    let forecast = vec![
        ForecastData {
            date: "Tomorrow".to_string(),
            temperature_max: 25.0,
            temperature_min: 18.0,
            description: "Sunny".to_string(),
            icon: "01d".to_string(),
        },
        ForecastData {
            date: "Day After".to_string(),
            temperature_max: 23.0,
            temperature_min: 16.0,
            description: "Partly cloudy".to_string(),
            icon: "02d".to_string(),
        },
        ForecastData {
            date: "3 Days".to_string(),
            temperature_max: 20.0,
            temperature_min: 14.0,
            description: "Light rain".to_string(),
            icon: "10d".to_string(),
        },
    ];

    Ok(WeatherResponse { current, forecast })
}

#[tauri::command]
async fn get_mock_air_quality(_city: String) -> Result<AirQualityData, String> {
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
            get_mock_weather,
            get_mock_air_quality
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
