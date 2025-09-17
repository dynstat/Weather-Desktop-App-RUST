import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { AirQualityData } from '@/types';
import { Activity, AlertTriangle, CheckCircle } from 'lucide-react';

interface AQICardProps {
  airQuality: AirQualityData;
}

const AQICard: React.FC<AQICardProps> = ({ airQuality }) => {
  const getAQIIcon = (aqi: number) => {
    if (aqi <= 50) return <CheckCircle className="h-6 w-6 text-green-400" />;
    if (aqi <= 100) return <Activity className="h-6 w-6 text-yellow-400" />;
    return <AlertTriangle className="h-6 w-6 text-red-400" />;
  };

  const getAQIBgColor = (aqi: number) => {
    if (aqi <= 50) return 'bg-green-500/20 border-green-500/30';
    if (aqi <= 100) return 'bg-yellow-500/20 border-yellow-500/30';
    if (aqi <= 150) return 'bg-orange-500/20 border-orange-500/30';
    if (aqi <= 200) return 'bg-red-500/20 border-red-500/30';
    if (aqi <= 300) return 'bg-purple-500/20 border-purple-500/30';
    return 'bg-red-800/20 border-red-800/30';
  };

  return (
    <Card className="glass-card border-white/20 shadow-premium">
      <CardHeader className="pb-4">
        <div className="flex items-center justify-between">
          <CardTitle className="text-xl font-semibold text-white">
            Air Quality Index
          </CardTitle>
          {getAQIIcon(airQuality.aqi)}
        </div>
      </CardHeader>
      
      <CardContent className="space-y-6">
        <div className="text-center">
          <div className={`inline-flex items-center justify-center w-20 h-20 rounded-full border-2 ${getAQIBgColor(airQuality.aqi)}`}>
            <span className="text-2xl font-bold text-white">{airQuality.aqi}</span>
          </div>
          <div className="mt-3">
            <h3 className="text-lg font-medium text-white">{airQuality.category}</h3>
          </div>
        </div>
        
        <div className="grid grid-cols-2 gap-3">
          <div className="p-3 rounded-lg bg-white/5 border border-emerald-500/20 hover:bg-emerald-500/10 transition-all duration-200">
            <div className="text-xs text-gray-400 uppercase tracking-wide">PM2.5</div>
            <div className="text-lg font-semibold text-white">{airQuality.pm25} μg/m³</div>
          </div>
          
          <div className="p-3 rounded-lg bg-white/5 border border-emerald-500/20 hover:bg-emerald-500/10 transition-all duration-200">
            <div className="text-xs text-gray-400 uppercase tracking-wide">PM10</div>
            <div className="text-lg font-semibold text-white">{airQuality.pm10} μg/m³</div>
          </div>
          
          <div className="p-3 rounded-lg bg-white/5 border border-emerald-500/20 hover:bg-emerald-500/10 transition-all duration-200">
            <div className="text-xs text-gray-400 uppercase tracking-wide">O₃</div>
            <div className="text-lg font-semibold text-white">{airQuality.o3} ppm</div>
          </div>
          
          <div className="p-3 rounded-lg bg-white/5 border border-emerald-500/20 hover:bg-emerald-500/10 transition-all duration-200">
            <div className="text-xs text-gray-400 uppercase tracking-wide">NO₂</div>
            <div className="text-lg font-semibold text-white">{airQuality.no2} ppm</div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default AQICard;
