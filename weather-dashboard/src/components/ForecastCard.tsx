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
    return `https://openweathermap.org/img/wn/${icon}@2x.png`;
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
                <img
                  src={getWeatherIcon(day.icon)}
                  alt={day.description}
                  className="h-10 w-10"
                />
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
