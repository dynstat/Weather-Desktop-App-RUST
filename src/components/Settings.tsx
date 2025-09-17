import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { AppSettings } from '@/types';
import { Save, MapPin, Thermometer, Eye } from 'lucide-react';

interface SettingsProps {
  settings: AppSettings;
  onSettingsChange: (settings: AppSettings) => void;
}

const Settings: React.FC<SettingsProps> = ({ settings, onSettingsChange }) => {
  const [localSettings, setLocalSettings] = useState<AppSettings>(settings);

  const handleSave = () => {
    onSettingsChange(localSettings);
  };

  const handleLatitudeChange = (latitude: number) => {
    setLocalSettings(prev => ({ ...prev, latitude }));
  };

  const handleLongitudeChange = (longitude: number) => {
    setLocalSettings(prev => ({ ...prev, longitude }));
  };

  const handleTemperatureUnitChange = (unit: 'celsius' | 'fahrenheit') => {
    setLocalSettings(prev => ({ ...prev, temperatureUnit: unit }));
  };

  const handleAirQualityToggle = (enabled: boolean) => {
    setLocalSettings(prev => ({ ...prev, enableAirQuality: enabled }));
  };

  return (
    <div className="space-y-6">
      <Card className="glass-card border-white/20 shadow-premium">
        <CardHeader>
          <CardTitle className="text-xl font-semibold text-white flex items-center space-x-2">
            <MapPin className="h-5 w-5 text-blue-400" />
            <span>Location Settings</span>
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Latitude
              </label>
              <input
                type="number"
                step="0.01"
                value={localSettings.latitude}
                onChange={(e) => handleLatitudeChange(parseFloat(e.target.value) || 0)}
                className="w-full px-4 py-2 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="52.52"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Longitude
              </label>
              <input
                type="number"
                step="0.01"
                value={localSettings.longitude}
                onChange={(e) => handleLongitudeChange(parseFloat(e.target.value) || 0)}
                className="w-full px-4 py-2 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="13.41"
              />
            </div>
          </div>
          <div className="text-xs text-gray-400">
            Example: Berlin (52.52°N, 13.41°E), London (51.51°N, -0.13°W), New York (40.71°N, -74.01°W)
          </div>
        </CardContent>
      </Card>

      <Card className="glass-card border-white/20 shadow-premium">
        <CardHeader>
          <CardTitle className="text-xl font-semibold text-white flex items-center space-x-2">
            <Thermometer className="h-5 w-5 text-orange-400" />
            <span>Temperature Units</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex space-x-4">
            <Button
              variant={localSettings.temperatureUnit === 'celsius' ? 'default' : 'outline'}
              onClick={() => handleTemperatureUnitChange('celsius')}
              className={localSettings.temperatureUnit === 'celsius' ? 'bg-white/20 text-white' : 'text-gray-300 border-white/20'}
            >
              Celsius (°C)
            </Button>
            <Button
              variant={localSettings.temperatureUnit === 'fahrenheit' ? 'default' : 'outline'}
              onClick={() => handleTemperatureUnitChange('fahrenheit')}
              className={localSettings.temperatureUnit === 'fahrenheit' ? 'bg-white/20 text-white' : 'text-gray-300 border-white/20'}
            >
              Fahrenheit (°F)
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card className="glass-card border-white/20 shadow-premium">
        <CardHeader>
          <CardTitle className="text-xl font-semibold text-white flex items-center space-x-2">
            <Eye className="h-5 w-5 text-green-400" />
            <span>Display Options</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-white">Show Air Quality Data</div>
              <div className="text-xs text-gray-400">Display air quality information on dashboard</div>
            </div>
            <Button
              variant={localSettings.enableAirQuality ? 'default' : 'outline'}
              onClick={() => handleAirQualityToggle(!localSettings.enableAirQuality)}
              className={localSettings.enableAirQuality ? 'bg-white/20 text-white' : 'text-gray-300 border-white/20'}
            >
              {localSettings.enableAirQuality ? 'Enabled' : 'Disabled'}
            </Button>
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button
          onClick={handleSave}
          className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 flex items-center space-x-2"
        >
          <Save className="h-4 w-4" />
          <span>Save Settings</span>
        </Button>
      </div>
    </div>
  );
};

export default Settings;
