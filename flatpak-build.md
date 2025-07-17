# Flatpak Build Instructions

## Prerequisites

Install the required tools:

```bash
sudo apt install flatpak flatpak-builder
# Add Flathub repository
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
# Install required runtimes
flatpak install flathub org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
```

Install the Flatpak Cargo Generator:

```bash
# Download the flatpak-cargo-generator.py script
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
chmod +x flatpak-cargo-generator.py
sudo mv flatpak-cargo-generator.py /usr/local/bin/
```

## Generate Cargo Sources

Before building, you need to generate the cargo sources file:

```bash
./generate-cargo-sources.sh
```

This creates `cargo-sources.json` which contains all the Rust dependencies needed for offline building.

## Build the Flatpak

```bash
# Build the application
flatpak-builder --force-clean build-dir sh.tangled.one.weird.nandi.weather-applet.yml

# Install locally for testing
flatpak-builder --user --install --force-clean build-dir sh.tangled.one.weird.nandi.weather-applet.yml

# Run the application
flatpak run sh.tangled.one.weird.nandi.weather-applet
```

## Create a Flatpak Bundle

To create a distributable bundle:

```bash
flatpak build-bundle ~/.local/share/flatpak/repo weather-applet.flatpak sh.tangled.one.weird.nandi.weather-applet
```

## Notes

- The manifest includes network permissions for weather API access and IP geolocation
- Wayland and X11 socket access for GUI display
- Configuration directory access for storing settings
- Desktop portal access for potential future location services integration
- The build uses offline mode to ensure reproducible builds