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

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenWeatherResponse {
    main: OpenWeatherMain,
    weather: Vec<OpenWeatherWeather>,
    wind: OpenWeatherWind,
    name: String,
    sys: OpenWeatherSys,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherMain {
    temp: f64,
    humidity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherWeather {
    description: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherWind {
    speed: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherSys {
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirVisualResponse {
    data: AirVisualData,
}

#[derive(Debug, Serialize, Deserialize)]
struct AirVisualData {
    current: AirVisualCurrent,
}

#[derive(Debug, Serialize, Deserialize)]
struct AirVisualCurrent {
    pollution: AirVisualPollution,
}

#[derive(Debug, Serialize, Deserialize)]
struct AirVisualPollution {
    aqius: i32,
    mainus: String,
    aqicn: i32,
    maincn: String,
}

#[tauri::command]
async fn get_weather(city: String) -> Result<WeatherResponse, String> {
    let api_key = "YOUR_OPENWEATHER_API_KEY"; // Replace with your actual API key
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch weather data: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Weather API error: {}", response.status()));
    }

    let weather_data: OpenWeatherResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse weather data: {}", e))?;

    // For demo purposes, create mock forecast data
    let forecast = vec![
        ForecastData {
            date: "Tomorrow".to_string(),
            temperature_max: weather_data.main.temp + 2.0,
            temperature_min: weather_data.main.temp - 2.0,
            description: weather_data.weather[0].description.clone(),
            icon: weather_data.weather[0].icon.clone(),
        },
        ForecastData {
            date: "Day After".to_string(),
            temperature_max: weather_data.main.temp + 1.0,
            temperature_min: weather_data.main.temp - 3.0,
            description: "Partly cloudy".to_string(),
            icon: "02d".to_string(),
        },
        ForecastData {
            date: "3 Days".to_string(),
            temperature_max: weather_data.main.temp - 1.0,
            temperature_min: weather_data.main.temp - 5.0,
            description: "Light rain".to_string(),
            icon: "10d".to_string(),
        },
    ];

    let current = WeatherData {
        temperature: weather_data.main.temp,
        humidity: weather_data.main.humidity,
        wind_speed: weather_data.wind.speed,
        description: weather_data.weather[0].description.clone(),
        icon: weather_data.weather[0].icon.clone(),
        city: weather_data.name,
        country: weather_data.sys.country,
    };

    Ok(WeatherResponse { current, forecast })
}

#[tauri::command]
async fn get_air_quality(city: String) -> Result<AirQualityData, String> {
    let api_key = "YOUR_AIRVISUAL_API_KEY"; // Replace with your actual API key
    let url = format!(
        "https://api.airvisual.com/v2/city?city={}&state=&country=&key={}",
        city, api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch air quality data: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Air quality API error: {}", response.status()));
    }

    let air_data: AirVisualResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse air quality data: {}", e))?;

    let aqi = air_data.data.current.pollution.aqius;
    let (category, color) = get_aqi_category(aqi);

    Ok(AirQualityData {
        aqi,
        category,
        color,
        pm25: 15.0, // Mock data
        pm10: 25.0,
        o3: 0.05,
        no2: 0.02,
        so2: 0.01,
        co: 0.5,
    })
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
            get_weather,
            get_air_quality,
            get_mock_weather,
            get_mock_air_quality
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
