# Simple weather info applet for cosmic

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_1.png">
</p>

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_2.png">
</p>

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_3.png">
</p>

## Installation

The applet can be installed using the following steps:

```sh
sudo apt install libxkbcommon-dev just
git clone https://github.com/cosmic-utils/cosmic-ext-applet-weather.git
cd cosmic-ext-applet-weather
just build
sudo just install
```

`libxkbcommon-dev` is required by `smithay-client-toolkit`

## Configuration

The applet includes a graphical configuration interface accessible by clicking on the weather icon in the panel. This opens a settings popup with multiple location options:

### Location Configuration Options

**1. City Search with Fuzzy Matching**
- Type a city name in the "Search City" field
- Get real-time fuzzy-matched results from OpenStreetMap's Nominatim API
- Click on any result to automatically populate coordinates
- Examples: "new york", "portland", "tokyo"

**2. Automatic Location Detection**
- Click "Get Current Location" to automatically detect your location using IP geolocation
- Uses your public IP address to determine approximate coordinates
- Displays detected city and region information

**3. Manual Coordinate Entry**
- Enter latitude and longitude values directly
- Useful for precise location specification
- Coordinates are validated when saved

### Temperature Unit
- Toggle between Celsius and Fahrenheit using the unit switcher
- Setting is saved and persists between app restarts

### Saving Configuration
- Click "Save" to apply your location and temperature unit settings
- The weather data refreshes automatically every minute
- Configuration is stored in the system's configuration directory

All settings are saved automatically and persist between application restarts.

## Uninstall

To uninstall files installed by `just install`, run:

```sh
sudo just uninstall
```
