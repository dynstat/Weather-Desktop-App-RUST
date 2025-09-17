export interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  description: string;
  icon: string;
  city: string;
  country: string;
}

export interface ForecastData {
  date: string;
  temperature_max: number;
  temperature_min: number;
  description: string;
  icon: string;
}

export interface AirQualityData {
  aqi: number;
  category: string;
  color: string;
  pm25: number;
  pm10: number;
  o3: number;
  no2: number;
  so2: number;
  co: number;
}

export interface WeatherResponse {
  current: WeatherData;
  forecast: ForecastData[];
}

export interface AppSettings {
  latitude: number;
  longitude: number;
  temperatureUnit: 'celsius' | 'fahrenheit';
  enableAirQuality: boolean;
}
