use cosmic::cosmic_config::{
    self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};

const CONFIG_VERSION: u64 = 1;

pub const APP_ID: &str = "sh.tangled.one.weird.nandi.weather-applet";
pub const SUN_ICON: &str = "sh.tangled.one.weird.nandi.weather-applet-sun-symbolic";
pub const MOON_ICON: &str = "sh.tangled.one.weird.nandi.weather-applet-moon-symbolic";

#[derive(Default, Debug, Clone, CosmicConfigEntry)]
pub struct WeatherConfig {
    pub latitude: f64,
    pub longitude: f64,
    pub use_fahrenheit: bool,
}

impl WeatherConfig {
    fn config_handler() -> Option<Config> {
        Config::new(APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> WeatherConfig {
        match Self::config_handler() {
            Some(config_handler) => WeatherConfig::get_entry(&config_handler)
                .map_err(|error| {
                    tracing::info!("Error whilst loading config: {:#?}", error);
                })
                .unwrap_or_default(),
            None => WeatherConfig::default(),
        }
    }
}
