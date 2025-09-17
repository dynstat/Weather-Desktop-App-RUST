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

// OpenWeatherMap API Response Structures
#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherResponse {
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

// OpenWeatherMap 5-day Forecast Response
#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherForecastResponse {
    list: Vec<OpenWeatherForecastItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenWeatherForecastItem {
    dt_txt: String,
    main: OpenWeatherMain,
    weather: Vec<OpenWeatherWeather>,
}

// AirVisual API Response Structures
#[derive(Debug, Serialize, Deserialize)]
struct AirVisualResponse {
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

// Real API Functions
#[tauri::command]
async fn get_weather(city: String) -> Result<WeatherResponse, String> {
    // You can get a free API key from https://openweathermap.org/api
    let api_key = "YOUR_OPENWEATHER_API_KEY"; // Replace with your actual API key
    
    if api_key == "YOUR_OPENWEATHER_API_KEY" {
        return Err("Please add your OpenWeatherMap API key to the Rust code".to_string());
    }
    
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

    // Get 5-day forecast
    let forecast_url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric",
        city, api_key
    );

    let forecast_response = reqwest::get(&forecast_url)
        .await
        .map_err(|e| format!("Failed to fetch forecast data: {}", e))?;

    if !forecast_response.status().is_success() {
        return Err(format!("Forecast API error: {}", forecast_response.status()));
    }

    let forecast_data: OpenWeatherForecastResponse = forecast_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse forecast data: {}", e))?;

    // Process forecast data - get next 3 days
    let mut forecast = Vec::new();
    let mut current_date = String::new();
    let mut day_count = 0;

    for item in &forecast_data.list {
        let date = item.dt_txt.split(' ').next().unwrap_or("");
        if date != current_date && day_count < 3 {
            current_date = date.to_string();
            day_count += 1;
            
            // Get max/min temp for the day
            let mut max_temp = item.main.temp;
            let mut min_temp = item.main.temp;
            let description = item.weather[0].description.clone();
            let icon = item.weather[0].icon.clone();

            // Find max/min for this day
            for other_item in &forecast_data.list {
                let other_date = other_item.dt_txt.split(' ').next().unwrap_or("");
                if other_date == date {
                    max_temp = max_temp.max(other_item.main.temp);
                    min_temp = min_temp.min(other_item.main.temp);
                }
            }

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
                description,
                icon,
            });
        }
    }

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
    // You can get a free API key from https://www.iqair.com/air-pollution-data-api
    let api_key = "YOUR_AIRVISUAL_API_KEY"; // Replace with your actual API key
    
    if api_key == "YOUR_AIRVISUAL_API_KEY" {
        return Err("Please add your AirVisual API key to the Rust code".to_string());
    }
    
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

    // Note: AirVisual API doesn't provide detailed pollutant data in the free tier
    // These are estimated values based on AQI
    let (pm25, pm10, o3, no2, so2, co) = estimate_pollutants_from_aqi(aqi);

    Ok(AirQualityData {
        aqi,
        category,
        color,
        pm25,
        pm10,
        o3,
        no2,
        so2,
        co,
    })
}

fn estimate_pollutants_from_aqi(aqi: i32) -> (f64, f64, f64, f64, f64, f64) {
    // Rough estimation based on AQI ranges
    let pm25 = match aqi {
        0..=50 => 12.0 + (aqi as f64 * 0.76),
        51..=100 => 50.0 + ((aqi - 50) as f64 * 0.5),
        101..=150 => 75.0 + ((aqi - 100) as f64 * 0.5),
        151..=200 => 100.0 + ((aqi - 150) as f64 * 0.4),
        201..=300 => 120.0 + ((aqi - 200) as f64 * 0.8),
        _ => 200.0 + ((aqi - 300) as f64 * 0.5),
    };

    let pm10 = pm25 * 1.5;
    let o3 = pm25 * 0.05;
    let no2 = pm25 * 0.02;
    let so2 = pm25 * 0.01;
    let co = pm25 * 0.1;

    (pm25, pm10, o3, no2, so2, co)
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
