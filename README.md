# Weather Desktop App in Tauri

A modern desktop weather application built with Tauri (Rust backend) and React frontend, featuring a premium dark theme with glassmorphism design.

## Screenshot
<img width="1919" height="1032" alt="image" src="https://github.com/user-attachments/assets/8a9bed9b-2494-440f-a7fe-31135dce9633" />

## Features

- **Real-time Weather Data**: Current weather conditions with temperature, humidity, and wind speed
- **3-Day Forecast**: Extended weather forecast with daily predictions
- **Air Quality Index**: Comprehensive air quality monitoring with AQI categories
- **Premium UI/UX**: Dark theme with glassmorphism effects and smooth animations
- **Responsive Design**: Clean resizing for different window sizes
- **Settings Management**: Customizable location coordinates and temperature units
- **Modern Components**: Built with shadcn/ui and TailwindCSS

## Tech Stack

- **Backend**: Rust with Tauri
- **Frontend**: React + TypeScript
- **Styling**: TailwindCSS + shadcn/ui
- **Icons**: Lucide React
- **APIs**: Open-Meteo (free weather API) with mock air quality data

## Getting Started

### Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable version)
- npm or yarn

### Installation

1. Clone the repository:
```bash
git clone https://github.com/dynstat/Weather-Desktop-App-RUST
cd Weather-Desktop-App-RUST
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

The application uses Open-Meteo API for weather data and mock data for air quality:

### Weather API Integration

- **Open-Meteo API**: Free, no API key required
- **Real-time weather data**: Temperature, humidity, wind speed, precipitation, cloud cover
- **3-day forecast**: Generated from hourly data
- **Automatic fallback**: Falls back to mock data if API fails

### Air Quality Data

- **Mock data**: Currently uses simulated air quality data
- **AQI categories**: Good, Moderate, Unhealthy for Sensitive Groups, Unhealthy, Very Unhealthy, Hazardous
- **Pollutant levels**: PM2.5, PM10, O3, NO2, SO2, CO

### No API Keys Required

The app works out of the box with real weather data from Open-Meteo and mock air quality data, so no API keys are needed for basic functionality.

## Features Overview

### Dashboard
- Current weather display with temperature, conditions, and location
- Air quality index with color-coded categories
- 3-day weather forecast
- Responsive grid layout

### Settings
- Location coordinates (latitude/longitude)
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
2. Modify the Rust backend in `src-tauri/src/lib.rs` to fetch new data from Open-Meteo API
3. Update the `WeatherCard` component to display new data

### Styling Modifications

- Global styles: `src/App.css`
- Component styles: Individual component files
- Tailwind configuration: `tailwind.config.js`

## Development

### Available Scripts

- `npm run tauri dev` - Start development server
- `npm run tauri build` - Build for production
- `npm run build` - Build frontend only
- `npm run preview` - Preview built frontend

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
