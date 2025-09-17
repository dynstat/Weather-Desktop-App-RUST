# API Setup Guide

This Weather Dashboard supports real-time weather and air quality data from external APIs. Follow this guide to set up your API keys.

## 🌤️ OpenWeatherMap API (Weather Data)

### Step 1: Get Your API Key
1. Visit [OpenWeatherMap](https://openweathermap.org/api)
2. Sign up for a free account
3. Go to your API keys section
4. Copy your API key

### Step 2: Configure the API Key
1. Open `src-tauri/src/lib.rs`
2. Find line 126: `let api_key = "YOUR_OPENWEATHER_API_KEY";`
3. Replace `"YOUR_OPENWEATHER_API_KEY"` with your actual API key
4. Example: `let api_key = "abc123def456ghi789";`

### Step 3: Test the API
- The app will automatically try the real API first
- If the API key is not configured, it will fallback to mock data
- Check the browser console for any error messages

## 🌬️ AirVisual API (Air Quality Data)

### Step 1: Get Your API Key
1. Visit [AirVisual (IQAir)](https://www.iqair.com/air-pollution-data-api)
2. Sign up for a free account
3. Go to your API keys section
4. Copy your API key

### Step 2: Configure the API Key
1. Open `src-tauri/src/lib.rs`
2. Find line 228: `let api_key = "YOUR_AIRVISUAL_API_KEY";`
3. Replace `"YOUR_AIRVISUAL_API_KEY"` with your actual API key
4. Example: `let api_key = "xyz789uvw456rst123";`

## 🔧 API Features

### OpenWeatherMap (Free Tier)
- ✅ Current weather conditions
- ✅ 5-day weather forecast
- ✅ Temperature, humidity, wind speed
- ✅ Weather descriptions and icons
- ✅ 1,000 API calls per day

### AirVisual (Free Tier)
- ✅ Air Quality Index (AQI)
- ✅ AQI categories and colors
- ✅ Basic pollutant estimates
- ✅ 10,000 API calls per month

## 🚀 Usage

Once configured, the app will:
1. **Try real APIs first** - Fetch live data from OpenWeatherMap and AirVisual
2. **Fallback to mock data** - If APIs fail or keys are missing
3. **Show error messages** - If both real and mock data fail

## 🔒 Security Notes

- **Never commit API keys to version control**
- **Keep your API keys private**
- **Monitor your API usage** to avoid exceeding limits
- **Consider using environment variables** for production deployments

## 🆘 Troubleshooting

### Common Issues:
1. **"Please add your API key"** - API key not configured
2. **"API error: 401"** - Invalid API key
3. **"API error: 429"** - Rate limit exceeded
4. **"Failed to fetch"** - Network connectivity issue

### Solutions:
1. Verify your API key is correct
2. Check your internet connection
3. Wait if you've hit rate limits
4. Use mock data for testing

## 📊 Data Sources

- **Weather**: OpenWeatherMap (https://openweathermap.org/)
- **Air Quality**: AirVisual by IQAir (https://www.iqair.com/)
- **Icons**: OpenWeatherMap weather icons
- **Fallback**: Mock data for demo purposes

## 🎯 Next Steps

1. Get your API keys from both services
2. Update the Rust code with your keys
3. Restart the application
4. Enjoy real-time weather and air quality data!

---

**Note**: The app works perfectly with mock data for demonstration purposes. Real APIs are optional but provide live, accurate data.
