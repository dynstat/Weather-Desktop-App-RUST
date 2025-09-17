import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { WeatherData } from '@/types';
import { Droplets, Wind, MapPin } from 'lucide-react';

interface WeatherCardProps {
  weather: WeatherData;
  temperatureUnit: 'celsius' | 'fahrenheit';
}

const WeatherCard: React.FC<WeatherCardProps> = ({ weather, temperatureUnit }) => {
  const convertTemperature = (temp: number) => {
    if (temperatureUnit === 'fahrenheit') {
      return (temp * 9/5) + 32;
    }
    return temp;
  };

  const getWeatherIcon = (icon: string) => {
    // Check if it's an emoji (Open-Meteo) or an icon code (OpenWeatherMap)
    if (icon.includes('http') || icon.includes('@')) {
      return icon; // Already a full URL
    } else if (icon.match(/[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/u)) {
      return icon; // It's an emoji, return as is
    } else {
      return `https://openweathermap.org/img/wn/${icon}@2x.png`; // Fallback to OpenWeatherMap
    }
  };

  return (
    <Card className="glass-card border-white/20 shadow-premium">
      <CardHeader className="pb-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <MapPin className="h-5 w-5 text-blue-400" />
            <CardTitle className="text-xl font-semibold text-white">
              {weather.city}, {weather.country}
            </CardTitle>
          </div>
          <div className="text-right">
            <div className="text-4xl font-bold text-white">
              {Math.round(convertTemperature(weather.temperature))}°
            </div>
            <div className="text-sm text-gray-300 uppercase tracking-wide">
              {temperatureUnit === 'celsius' ? 'C' : 'F'}
            </div>
          </div>
        </div>
      </CardHeader>
      
      <CardContent className="space-y-6">
        <div className="flex items-center justify-center">
          {weather.icon.match(/[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/u) ? (
            <div className="text-8xl">{weather.icon}</div>
          ) : (
            <img
              src={getWeatherIcon(weather.icon)}
              alt={weather.description}
              className="h-24 w-24"
            />
          )}
        </div>
        
        <div className="text-center">
          <h3 className="text-lg font-medium text-white capitalize">
            {weather.description}
          </h3>
        </div>
        
        <div className="grid grid-cols-2 gap-4">
          <div className="flex items-center space-x-3 p-3 rounded-lg bg-white/5 border border-white/10">
            <Droplets className="h-5 w-5 text-blue-400" />
            <div>
              <div className="text-sm text-gray-400">Humidity</div>
              <div className="text-lg font-semibold text-white">{weather.humidity}%</div>
            </div>
          </div>
          
          <div className="flex items-center space-x-3 p-3 rounded-lg bg-white/5 border border-white/10">
            <Wind className="h-5 w-5 text-green-400" />
            <div>
              <div className="text-sm text-gray-400">Wind Speed</div>
              <div className="text-lg font-semibold text-white">{weather.wind_speed} m/s</div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default WeatherCard;
