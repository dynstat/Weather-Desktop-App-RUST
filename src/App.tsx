import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Sidebar from './components/Sidebar';
import Header from './components/Header';
import WeatherCard from './components/WeatherCard';
import AQICard from './components/AQICard';
import ForecastCard from './components/ForecastCard';
import Settings from './components/Settings';
import { WeatherResponse, AirQualityData, AppSettings } from './types';
import './App.css';

function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [weatherData, setWeatherData] = useState<WeatherResponse | null>(null);
  const [airQualityData, setAirQualityData] = useState<AirQualityData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [settings, setSettings] = useState<AppSettings>({
    city: 'London',
    temperatureUnit: 'celsius',
    enableAirQuality: true,
  });

  const fetchWeatherData = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // Try real APIs first, fallback to mock data if API keys are not configured
      let weather: WeatherResponse;
      let airQuality: AirQualityData | null = null;

      try {
        weather = await invoke<WeatherResponse>('get_weather', { 
          city: settings.city 
        });
      } catch (weatherErr) {
        // Fallback to mock data if real API fails
        console.log('Real weather API failed, using mock data:', weatherErr);
        weather = await invoke<WeatherResponse>('get_mock_weather', { 
          city: settings.city 
        });
      }

      setWeatherData(weather);

      if (settings.enableAirQuality) {
        try {
          airQuality = await invoke<AirQualityData>('get_air_quality', { 
            city: settings.city 
          });
        } catch (airErr) {
          // Fallback to mock data if real API fails
          console.log('Real air quality API failed, using mock data:', airErr);
          airQuality = await invoke<AirQualityData>('get_mock_air_quality', { 
            city: settings.city 
          });
        }
        setAirQualityData(airQuality);
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchWeatherData();
  }, [settings.city, settings.enableAirQuality]);

  const handleSettingsChange = (newSettings: AppSettings) => {
    setSettings(newSettings);
  };

  const renderDashboard = () => {
    if (isLoading) {
      return (
        <div className="flex items-center justify-center h-64">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-400"></div>
        </div>
      );
    }

    if (error) {
      return (
        <div className="text-center py-12">
          <div className="text-red-400 text-lg mb-2">Error loading data</div>
          <div className="text-gray-400">{error}</div>
          <button
            onClick={fetchWeatherData}
            className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            Retry
          </button>
        </div>
      );
    }

    if (!weatherData) {
      return (
        <div className="text-center py-12">
          <div className="text-gray-400">No weather data available</div>
        </div>
      );
    }

    return (
      <div className="space-y-6">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <WeatherCard 
            weather={weatherData.current} 
            temperatureUnit={settings.temperatureUnit}
          />
          {settings.enableAirQuality && airQualityData && (
            <AQICard airQuality={airQualityData} />
          )}
        </div>
        
        <ForecastCard 
          forecast={weatherData.forecast} 
          temperatureUnit={settings.temperatureUnit}
        />
      </div>
    );
  };

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return renderDashboard();
      case 'weather':
        return weatherData ? (
          <WeatherCard 
            weather={weatherData.current} 
            temperatureUnit={settings.temperatureUnit}
          />
        ) : null;
      case 'air-quality':
        return airQualityData && settings.enableAirQuality ? (
          <AQICard airQuality={airQualityData} />
        ) : (
          <div className="text-center py-12">
            <div className="text-gray-400">Air quality data is disabled in settings</div>
          </div>
        );
      case 'settings':
        return (
          <Settings 
            settings={settings} 
            onSettingsChange={handleSettingsChange}
          />
        );
      default:
        return renderDashboard();
    }
  };

  return (
    <div className="h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 flex">
      <Sidebar activeTab={activeTab} onTabChange={setActiveTab} />
      
      <div className="flex-1 flex flex-col">
        <Header 
          city={settings.city}
          onRefresh={fetchWeatherData}
          onSearch={() => setActiveTab('settings')}
          isLoading={isLoading}
        />
        
        <main className="flex-1 p-6 overflow-auto">
          <div className="max-w-7xl mx-auto">
            {renderContent()}
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;