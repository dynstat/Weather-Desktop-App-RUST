import React from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { ForecastData } from '@/types';

interface ForecastCardProps {
  forecast: ForecastData[];
  temperatureUnit: 'celsius' | 'fahrenheit';
}

const ForecastCard: React.FC<ForecastCardProps> = ({ forecast, temperatureUnit }) => {
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
      <CardContent className="p-6">
        <h3 className="text-xl font-semibold text-white mb-6">3-Day Forecast</h3>
        
        <div className="space-y-4">
          {forecast.map((day, index) => (
            <div
              key={index}
              className="flex items-center justify-between p-4 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-all duration-200"
            >
              <div className="flex items-center space-x-4">
                <div className="text-sm font-medium text-gray-300 min-w-[80px]">
                  {day.date}
                </div>
                {day.icon.match(/[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/u) ? (
                  <div className="text-3xl">{day.icon}</div>
                ) : (
                  <img
                    src={getWeatherIcon(day.icon)}
                    alt={day.description}
                    className="h-10 w-10"
                  />
                )}
                <div>
                  <div className="text-sm text-white capitalize">
                    {day.description}
                  </div>
                </div>
              </div>
              
              <div className="flex items-center space-x-3">
                <div className="text-right">
                  <div className="text-lg font-semibold text-white">
                    {Math.round(convertTemperature(day.temperature_max))}°
                  </div>
                  <div className="text-sm text-gray-400">
                    {Math.round(convertTemperature(day.temperature_min))}°
                  </div>
                </div>
                <div className="text-xs text-gray-400">
                  {temperatureUnit === 'celsius' ? 'C' : 'F'}
                </div>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
};

export default ForecastCard;
