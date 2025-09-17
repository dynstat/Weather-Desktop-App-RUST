# Weather Desktop App in Tauri

A premium desktop application built with Tauri (Rust backend) and React frontend, featuring a modern dark theme with glassmorphism design.

## Screenshot
<img width="1919" height="1032" alt="image" src="https://github.com/user-attachments/assets/8a9bed9b-2494-440f-a7fe-31135dce9633" />


## Features

- **Real-time Weather Data**: Current weather conditions with temperature, humidity, and wind speed
- **3-Day Forecast**: Extended weather forecast with daily predictions
- **Air Quality Index**: Comprehensive air quality monitoring with AQI categories
- **Premium UI/UX**: Dark theme with glassmorphism effects and smooth animations
- **Responsive Design**: Clean resizing for different window sizes
- **Settings Management**: Customizable city selection and temperature units
- **Modern Components**: Built with shadcn/ui and TailwindCSS

## Tech Stack

- **Backend**: Rust with Tauri
- **Frontend**: React + TypeScript
- **Styling**: TailwindCSS + shadcn/ui
- **Icons**: Lucide React
- **APIs**: OpenWeatherMap & AirVisual (with mock data for demo)

## Getting Started

### Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable version)
- npm or yarn

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd weather-dashboard
```

2. Install dependencies:
```bash
npm install
```

3. Run the development server:
```bash
npm run tauri dev
```

### Building for Production

```bash
npm run tauri build
```

## Project Structure

```
weather-dashboard/
├── src/
│   ├── components/
│   │   ├── ui/           # shadcn/ui components
│   │   ├── WeatherCard.tsx
│   │   ├── AQICard.tsx
│   │   ├── ForecastCard.tsx
│   │   ├── Sidebar.tsx
│   │   ├── Header.tsx
│   │   └── Settings.tsx
│   ├── types/
│   │   └── index.ts      # TypeScript type definitions
│   ├── lib/
│   │   └── utils.ts      # Utility functions
│   ├── App.tsx           # Main application component
│   └── App.css           # Global styles with glassmorphism
├── src-tauri/
│   ├── src/
│   │   └── lib.rs        # Rust backend with API endpoints
│   └── Cargo.toml        # Rust dependencies
└── README.md
```

## API Configuration

The application includes both real API integration and mock data for demonstration:

### Real API Integration

The app automatically tries real APIs first and falls back to mock data if API keys are not configured.

#### To use real weather and air quality data:

1. **Get API keys:**
   - [OpenWeatherMap API](https://openweathermap.org/api) - Free tier: 1,000 calls/day
   - [AirVisual API](https://www.iqair.com/air-pollution-data-api) - Free tier: 10,000 calls/month

2. **Update the API keys in `src-tauri/src/lib.rs`:**
```rust
// Line 126 - Weather API
let api_key = "YOUR_ACTUAL_OPENWEATHER_API_KEY";

// Line 228 - Air Quality API  
let api_key = "YOUR_ACTUAL_AIRVISUAL_API_KEY";
```

3. **The app automatically handles the rest:**
   - Tries real APIs first
   - Falls back to mock data if APIs fail
   - Shows appropriate error messages

### Mock Data

The application includes comprehensive mock data that works out of the box, allowing you to see the full functionality without requiring API keys.

### API Features

- **OpenWeatherMap**: Current weather, 5-day forecast, temperature, humidity, wind speed
- **AirVisual**: Air Quality Index (AQI), pollutant levels, health categories
- **Automatic fallback**: Seamless transition between real and mock data

## Features Overview

### Dashboard
- Current weather display with temperature, conditions, and location
- Air quality index with color-coded categories
- 3-day weather forecast
- Responsive grid layout

### Settings
- City selection
- Temperature unit toggle (Celsius/Fahrenheit)
- Air quality data enable/disable
- Persistent settings storage

### Navigation
- Sidebar navigation with smooth transitions
- Header with refresh and search functionality
- Tab-based content switching

## Styling

The application uses a premium dark theme with glassmorphism effects:

- **Background**: Gradient from slate-900 to purple-900
- **Cards**: Semi-transparent with backdrop blur
- **Borders**: Subtle white borders with opacity
- **Shadows**: Premium shadow effects
- **Animations**: Smooth transitions and hover effects

## Customization

### Adding New Weather Data Points

1. Update the `WeatherData` interface in `src/types/index.ts`
2. Modify the Rust backend in `src-tauri/src/lib.rs`
3. Update the `WeatherCard` component to display new data

### Styling Modifications

- Global styles: `src/App.css`
- Component styles: Individual component files
- Tailwind configuration: `tailwind.config.js`

## Development

### Available Scripts

- `npm run tauri dev` - Start development server
- `npm run tauri build` - Build for production
- `npm run lint` - Run ESLint
- `npm run type-check` - Run TypeScript type checking

### Code Structure

- **Components**: Reusable UI components with TypeScript
- **Types**: Centralized type definitions
- **Utils**: Shared utility functions
- **Backend**: Rust commands for API integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Support

For support or questions, please open an issue in the repository.
