# Setup Instructions

## Quick Start

1. **Install Dependencies**
   ```bash
   npm install
   ```

2. **Run the Application**
   ```bash
   npm run tauri dev
   ```

## API Configuration (Optional)

The app works with mock data by default. To use real weather data:

### OpenWeatherMap API
1. Visit [OpenWeatherMap](https://openweathermap.org/api)
2. Sign up for a free account
3. Get your API key
4. Update `src-tauri/src/lib.rs` line 99:
   ```rust
   let api_key = "YOUR_ACTUAL_API_KEY_HERE";
   ```

### AirVisual API
1. Visit [AirVisual](https://www.iqair.com/air-pollution-data-api)
2. Sign up for a free account
3. Get your API key
4. Update `src-tauri/src/lib.rs` line 158:
   ```rust
   let api_key = "YOUR_ACTUAL_API_KEY_HERE";
   ```

### Switch to Real APIs
In `src/App.tsx`, change the function calls:
```typescript
// From mock data:
const weather = await invoke<WeatherResponse>('get_mock_weather', { city: settings.city });

// To real data:
const weather = await invoke<WeatherResponse>('get_weather', { city: settings.city });
```

## Features

- ✅ Premium dark theme with glassmorphism
- ✅ Current weather display
- ✅ 3-day forecast
- ✅ Air quality index
- ✅ Settings management
- ✅ Responsive design
- ✅ Smooth animations
- ✅ Mock data for demo

## Troubleshooting

### Build Issues
- Ensure Rust is installed: `rustup --version`
- Update Rust: `rustup update`
- Clean build: `cargo clean` in `src-tauri/`

### Development Issues
- Clear node modules: `rm -rf node_modules && npm install`
- Check Node version: `node --version` (should be 16+)

## Project Structure

```
src/
├── components/     # React components
├── types/         # TypeScript definitions
├── lib/           # Utilities
└── App.tsx        # Main app

src-tauri/
├── src/lib.rs     # Rust backend
└── Cargo.toml     # Dependencies
```
