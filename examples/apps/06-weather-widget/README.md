# App 6: Weather Widget 🌤️

**Complexity**: Simple
**Lines**: ~130
**Packages**: None (UI demo - jounce-http integration coming soon!)
**Time to Build**: 50 minutes

---

## 📖 Description

A beautiful weather widget application demonstrating:
- **Current Weather Display**: Temperature, conditions, feels-like
- **Weather Metrics**: Humidity, wind, pressure, visibility
- **5-Day Forecast**: Daily temperature ranges with icons
- **Weather Details**: Sunrise/sunset, UV index, air quality
- **Gradient Design**: Purple gradient background
- **Smooth Animations**: Floating weather icon, hover effects

---

## ✨ Features

- ✅ Large animated weather icon (floating animation)
- ✅ Current temperature display (72°F)
- ✅ Weather condition and "feels like" text
- ✅ 4 weather metrics in card grid (humidity, wind, pressure, visibility)
- ✅ 5-day forecast with emoji icons
- ✅ Hover effects on forecast days (color change)
- ✅ Weather details section (sunrise, sunset, UV, air quality)
- ✅ Responsive mobile design
- ✅ Gradient background (purple theme)

---

## 🎯 What This App Tests

### Language Features
- [x] **JSX grid layouts** - Metrics and forecast grids
- [x] **Emoji rendering** - Weather icons
- [x] **Temperature display** - Large numeric values
- [x] **Data cards** - Metric cards with icons and values

### UI Patterns
- [x] **Card-based design** - Weather card container
- [x] **CSS Grid** - 2-column metrics, 5-column forecast
- [x] **Gradient backgrounds** - Purple weather theme
- [x] **Hover animations** - Forecast day hover effects
- [x] **Floating animation** - Weather icon animation
- [x] **Responsive grids** - Mobile-friendly layout

### Future Enhancements
- [ ] **API integration** - OpenWeatherMap API
- [ ] **jounce-http** - Fetch weather data
- [ ] **Location detection** - Geolocation API
- [ ] **Unit toggle** - Celsius/Fahrenheit switch
- [ ] **Hourly forecast** - 24-hour temperature graph
- [ ] **Weather alerts** - Severe weather warnings

---

## 🚀 How to Build

### Step 1: Compile the App

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile app 06
cargo run -- compile examples/apps/06-weather-widget/main.jnc
```

**Expected output:**
```
✓ Compiled examples/apps/06-weather-widget/main.jnc
✓ Generated dist/client.js
✓ Generated dist/server.js
✓ Generated dist/index.html
```

---

## 🚢 How to Deploy

### Method 1: Production Server (Recommended)

```bash
# Start the Node.js server
cd dist
node server.js
```

**Then open:** http://localhost:3000

**What you should see:**
- Purple gradient background
- White weather card with rounded corners
- San Francisco location header
- Large sun emoji with floating animation
- 72°F temperature in large font
- "Sunny" condition text
- 4 weather metrics with emoji icons
- 5-day forecast row with daily temps
- Weather details section at bottom

---

### Method 2: HMR Dev Server (Live Reload)

```bash
# From the Jounce root directory
node scripts/hmr-server.js
```

**Then open:** http://localhost:3000

---

### Method 3: Static File (Quick Test)

```bash
cd dist
open index.html  # macOS
```

---

## 📸 What You Should See

### Browser View

```
┌─────────────────────────────────────────────────┐
│          San Francisco                          │
│  Monday, October 25, 2025 • 3:45 PM            │
├─────────────────────────────────────────────────┤
│                                                 │
│                   ☀️                            │
│                   72°F                          │
│                   Sunny                         │
│              Feels like 70°F                    │
│                                                 │
├─────────────────────────────────────────────────┤
│  💧 Humidity      💨 Wind Speed                 │
│     45%             12 mph                      │
│                                                 │
│  🌡️ Pressure       👁️ Visibility               │
│     30.2 in          10 mi                      │
├─────────────────────────────────────────────────┤
│  5-Day Forecast                                 │
│  ⛅ 🌤️ 🌧️ ⛈️ 🌤️                                  │
│  75° 78° 68° 65° 73°                            │
├─────────────────────────────────────────────────┤
│  Sunrise: 6:42 AM    Sunset: 6:18 PM           │
│  UV Index: 7 (High)  Air Quality: Good         │
└─────────────────────────────────────────────────┘
```

**Visual Features:**
- ✅ Purple gradient background
- ✅ White rounded weather card
- ✅ Floating sun icon animation
- ✅ Large temperature display
- ✅ Grid of 4 metric cards
- ✅ 5-day forecast with hover effects
- ✅ Weather details in footer

---

## 💡 Key Concepts

### 1. Grid Layout Pattern

```jounce
<div class="weather-metrics">
    <div class="metric">...</div>
    <div class="metric">...</div>
    <div class="metric">...</div>
    <div class="metric">...</div>
</div>
```

CSS Grid creates 2x2 layout automatically.

### 2. Large Temperature Display

```jounce
<div class="temperature">
    <span class="temp-value">72</span>
    <span class="temp-unit">°F</span>
</div>
```

Flexbox aligns number and unit at baseline.

### 3. Forecast Grid

```jounce
<div class="forecast-days">
    <div class="forecast-day">
        <p class="day-name">Tue</p>
        <div class="day-icon">⛅</div>
        <p class="day-temp">75° / 62°</p>
    </div>
    // ... 4 more days
</div>
```

5-column grid for daily forecasts.

### 4. Floating Animation

```css
@keyframes float {
    0%, 100% { transform: translateY(0px); }
    50% { transform: translateY(-10px); }
}

.weather-icon {
    animation: float 3s ease-in-out infinite;
}
```

Weather icon floats up and down smoothly.

### 5. Hover Color Change

```css
.forecast-day:hover {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.forecast-day:hover .day-name,
.forecast-day:hover .day-temp {
    color: white;
}
```

Forecast days turn purple on hover.

---

## 📚 Learning Outcomes

After studying this app, you should understand:

1. ✅ How to create data-rich dashboard layouts
2. ✅ How to use CSS Grid for metric cards
3. ✅ How to display large numeric values
4. ✅ How to create floating animations
5. ✅ How to implement hover state changes
6. ✅ How to structure weather data display

---

## 🔄 Variations to Try

**Easy**:
- Change location to your city
- Use different emoji weather icons
- Change temperature units (°C instead of °F)
- Add more forecast days (7-day or 10-day)

**Medium**:
- Add hourly forecast (24 hours)
- Add temperature graph with SVG
- Add weather radar/map section
- Implement dark mode toggle

**Hard**:
- Integrate OpenWeatherMap API with jounce-http
- Add location search with autocomplete
- Add multiple city comparison
- Add weather alerts and notifications
- Implement unit conversion toggle

---

## 📝 Code Walkthrough

### Line-by-Line Explanation

```jounce
// Lines 8-11: Location header
<div class="location-header">
    <h1 class="city-name">San Francisco</h1>
    <p class="date-time">Monday, October 25, 2025 • 3:45 PM</p>
</div>
// - City name and current date/time
// - Future: Use DateTime stdlib for real-time updates

// Lines 13-19: Current weather
<div class="current-weather">
    <div class="weather-icon">☀️</div>
    <div class="temperature">
        <span class="temp-value">72</span>
        <span class="temp-unit">°F</span>
    </div>
    <p class="condition">Sunny</p>
    <p class="feels-like">Feels like 70°F</p>
</div>
// - Large weather icon with float animation
// - Temperature split into value and unit for styling
// - Condition and feels-like temperature

// Lines 22-52: Weather metrics
<div class="weather-metrics">
    <div class="metric">
        <div class="metric-icon">💧</div>
        <div class="metric-data">
            <span class="metric-label">Humidity</span>
            <span class="metric-value">45%</span>
        </div>
    </div>
    // ... 3 more metrics
</div>
// - 2x2 grid of weather metrics
// - Each metric has icon, label, and value
// - Hover animation on each card

// Lines 55-82: 5-day forecast
<div class="forecast-days">
    <div class="forecast-day">
        <p class="day-name">Tue</p>
        <div class="day-icon">⛅</div>
        <p class="day-temp">75° / 62°</p>
    </div>
    // ... 4 more days
</div>
// - 5-column grid
// - Each day shows name, icon, high/low temps
// - Hover effect changes background to purple gradient
```

---

## 🎓 Next Steps

After mastering this app, move on to:

**App 7: Image Gallery** - Image display, lightbox (future)

**App 8: Timer/Stopwatch** - Time tracking, animations (future)

---

## 🧪 Testing the Weather Widget

### Console Output

Open browser console to see:

```
App 6: Weather Widget started!
Future: API integration with OpenWeatherMap + jounce-http
Weather widget component created successfully!
```

### Visual Testing

Check that:
- ✅ Purple gradient background renders
- ✅ Weather card is white and centered
- ✅ Location header shows correctly
- ✅ Sun icon floats up and down
- ✅ Temperature displays 72°F
- ✅ 4 metric cards render in 2x2 grid
- ✅ Metric cards have hover effect
- ✅ 5 forecast days render in row
- ✅ Forecast days change color on hover
- ✅ Weather details section displays
- ✅ Responsive on mobile (stacks vertically)

---

## ✅ Success Criteria

This app is complete when:

- [x] Compiles without errors
- [x] Weather card renders centered
- [x] Current temperature displays
- [x] Weather icon animates
- [x] 4 metrics render in grid
- [x] 5-day forecast renders
- [x] Forecast hover effects work
- [x] Weather details display
- [x] Responsive on mobile
- [x] No console errors

---

## 🎨 Design Notes

### Weather App Inspiration

This design is inspired by modern weather apps:
- **iOS Weather**: Clean cards, large temperature
- **Weather.com**: Detailed metrics grid
- **AccuWeather**: Forecast row with icons
- **Dark Sky**: Minimalist gradient backgrounds

### Color Palette

```
Primary: #667eea (purple-blue)
Secondary: #764ba2 (purple)
Background: white (#ffffff)
Text Dark: #1f2937
Text Medium: #6b7280
Text Light: #9ca3af
Card BG: #f3f4f6 / #e5e7eb
```

---

## 🚧 Roadmap to Interactivity

**Phase 1** (Current): Static UI demonstration
- ✅ Weather card layout
- ✅ Temperature display
- ✅ Metrics grid
- ✅ 5-day forecast

**Phase 2** (Next): Add API integration
- [ ] Use `jounce-http` to fetch weather data
- [ ] Use OpenWeatherMap API
- [ ] Display real-time weather
- [ ] Update based on user location

**Phase 3** (Future): Advanced features
- [ ] Location search and autocomplete
- [ ] Hourly forecast graph
- [ ] Weather radar maps
- [ ] Weather alerts
- [ ] Multiple city tracking
- [ ] Unit toggle (°F/°C)

**Phase 4** (Future): Data visualization
- [ ] Temperature trend graphs
- [ ] Precipitation probability
- [ ] Wind direction compass
- [ ] UV index chart
- [ ] Historical weather data

---

## 🐛 Troubleshooting

### Issue: Weather icon not floating

**Cause**: CSS animation not applied
**Solution**: Check `.weather-icon { animation: float 3s ease-in-out infinite; }`

### Issue: Forecast not in a row

**Cause**: CSS Grid not applied
**Solution**: Verify `.forecast-days { display: grid; grid-template-columns: repeat(5, 1fr); }`

### Issue: Metrics stacked on desktop

**Cause**: Wrong grid columns
**Solution**: Check `.weather-metrics { grid-template-columns: repeat(2, 1fr); }`

---

## 📚 Resources

**Weather API Options:**
- OpenWeatherMap - Free tier with current + forecast
- WeatherAPI.com - 1M calls/month free
- AccuWeather API - 50 calls/day free

**Weather Icons:**
- Emoji (current approach - simple, works everywhere)
- Weather Icons font - 200+ weather glyphs
- Animated SVG icons - Smooth animations

---

## 🔍 Implementation Details

### Weather Data Structure (Future)

```jounce
struct WeatherData {
    location: string,
    temperature: float,
    feels_like: float,
    condition: string,
    humidity: int,
    wind_speed: float,
    pressure: float,
    visibility: float,
    sunrise: DateTime,
    sunset: DateTime,
    uv_index: int,
    air_quality: int,
}

let weather = signal<WeatherData>(default_weather);
```

### API Integration (Future)

```jounce
use jounce_http::{get};

async fn fetchWeather(city: string) -> Result<WeatherData, Error> {
    let api_key = "YOUR_API_KEY";
    let url = "https://api.openweathermap.org/data/2.5/weather?q=${city}&appid=${api_key}";

    let response = await get(url);
    let data = json.parse(response.body);

    return WeatherData {
        location: data.name,
        temperature: data.main.temp,
        feels_like: data.main.feels_like,
        condition: data.weather[0].description,
        humidity: data.main.humidity,
        wind_speed: data.wind.speed,
        // ...
    };
}
```

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0

**Next Phase**: Add jounce-http integration for real-time weather data
