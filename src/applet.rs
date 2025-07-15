use std::time::Duration;

use chrono::{Local, Timelike};
use cosmic::iced::window;
use cosmic::iced::platform_specific::shell::commands::popup;
use cosmic::cosmic_config::CosmicConfigEntry;

use crate::{
    config::{APP_ID, MOON_ICON, SUN_ICON, WeatherConfig},
    weather::get_location_forecast,
};

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Weather>(())
}

struct Weather {
    core: cosmic::app::Core,
    config: WeatherConfig,
    temperature: f64,
    popup: Option<window::Id>,
    latitude_input: String,
    longitude_input: String,
    use_fahrenheit: bool,
}

impl Weather {
    fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    fn format_temperature(&self, celsius: f64) -> String {
        if self.config.use_fahrenheit {
            format!("{:.0}°F", Self::celsius_to_fahrenheit(celsius))
        } else {
            format!("{:.0}°C", celsius)
        }
    }

    fn update_weather_data(&mut self) -> cosmic::app::Task<Message> {
        // Update config
        self.config = WeatherConfig::config();

        cosmic::Task::perform(
            get_location_forecast(self.config.latitude, self.config.longitude),
            |result| match result {
                Ok(temperature) => {
                    cosmic::action::Action::App(Message::UpdateTemperature(temperature))
                }
                Err(error) => {
                    tracing::error!("Failed to get location forecast: {error:?}");
                    cosmic::action::Action::App(Message::UpdateTemperature(0.0))
                }
            },
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    UpdateTemperature(f64),
    TogglePopup,
    LatitudeInput(String),
    LongitudeInput(String),
    ToggleTemperatureUnit(bool),
    SaveConfig,
    ClosePopup,
}

impl cosmic::Application for Weather {
    type Flags = ();
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;

    const APP_ID: &'static str = APP_ID;

    fn init(
        core: cosmic::app::Core,
        _flags: Self::Flags,
    ) -> (Self, cosmic::app::Task<Self::Message>) {
        let config = WeatherConfig::config();

        (
            Self {
                core,
                config: config.clone(),
                temperature: 0.0,
                popup: None,
                latitude_input: config.latitude.to_string(),
                longitude_input: config.longitude.to_string(),
                use_fahrenheit: config.use_fahrenheit,
            },
            cosmic::task::message(Message::Tick),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(60)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> cosmic::app::Task<Self::Message> {
        let mut tasks = vec![];

        tracing::info!("Received message: {:?}", message);

        match message {
            Message::Tick => tasks.push(self.update_weather_data()),
            Message::UpdateTemperature(temperature) => self.temperature = temperature,
            Message::TogglePopup => {
                if let Some(popup_id) = self.popup.take() {
                    tasks.push(popup::destroy_popup(popup_id));
                } else {
                    let new_id = window::Id::unique();
                    self.popup.replace(new_id);
                    
                    if let Some(main_window_id) = self.core.main_window_id() {
                        let popup_settings = self.core.applet.get_popup_settings(
                            main_window_id,
                            new_id,
                            Some((300, 200)),
                            None,
                            None
                        );
                        tasks.push(popup::get_popup(popup_settings));
                    }
                }
            }
            Message::LatitudeInput(value) => self.latitude_input = value,
            Message::LongitudeInput(value) => self.longitude_input = value,
            Message::ToggleTemperatureUnit(fahrenheit) => self.use_fahrenheit = fahrenheit,
            Message::SaveConfig => {
                if let (Ok(lat), Ok(lon)) = (
                    self.latitude_input.parse::<f64>(),
                    self.longitude_input.parse::<f64>()
                ) {
                    self.config.latitude = lat;
                    self.config.longitude = lon;
                    self.config.use_fahrenheit = self.use_fahrenheit;
                    
                    if let Some(config_handler) = cosmic::cosmic_config::Config::new(APP_ID, 1).ok() {
                        let _ = self.config.write_entry(&config_handler);
                    }
                    
                    tasks.push(self.update_weather_data());
                    
                    if let Some(popup_id) = self.popup.take() {
                        tasks.push(popup::destroy_popup(popup_id));
                    }
                }
            }
            Message::ClosePopup => {
                if let Some(popup_id) = self.popup.take() {
                    tasks.push(popup::destroy_popup(popup_id));
                }
            }
        }

        cosmic::Task::batch(tasks)
    }

    fn view(&self) -> cosmic::Element<Message> {
        self.view_window(self.core.main_window_id().unwrap_or(window::Id::unique()))
    }

    fn view_window(&self, id: window::Id) -> cosmic::Element<Message> {
        if Some(id) == self.popup {
            let content = cosmic::widget::column()
                .push(cosmic::widget::text("Weather Location Settings").size(16))
                .push(
                    cosmic::widget::row()
                        .push(cosmic::widget::text("Latitude:").width(cosmic::iced::Length::Fixed(80.0)))
                        .push(cosmic::widget::text_input("Enter latitude", &self.latitude_input)
                            .on_input(Message::LatitudeInput))
                        .spacing(10)
                        .align_y(cosmic::iced::Alignment::Center)
                )
                .push(
                    cosmic::widget::row()
                        .push(cosmic::widget::text("Longitude:").width(cosmic::iced::Length::Fixed(80.0)))
                        .push(cosmic::widget::text_input("Enter longitude", &self.longitude_input)
                            .on_input(Message::LongitudeInput))
                        .spacing(10)
                        .align_y(cosmic::iced::Alignment::Center)
                )
                .push(
                    cosmic::widget::row()
                        .push(cosmic::widget::text("Unit:").width(cosmic::iced::Length::Fixed(80.0)))
                        .push(cosmic::widget::toggler(self.use_fahrenheit)
                            .label("Fahrenheit")
                            .on_toggle(Message::ToggleTemperatureUnit))
                        .spacing(10)
                        .align_y(cosmic::iced::Alignment::Center)
                )
                .push(
                    cosmic::widget::row()
                        .push(cosmic::widget::button::standard("Cancel")
                            .on_press(Message::ClosePopup))
                        .push(cosmic::widget::button::suggested("Save")
                            .on_press(Message::SaveConfig))
                        .spacing(10)
                )
                .spacing(15)
                .padding(20);

            self.core.applet.popup_container(content).into()
        } else {
            let icon_name = match Local::now().hour() {
                6..18 => SUN_ICON,
                _ => MOON_ICON,
            };

            let icon = cosmic::widget::icon::from_name(icon_name)
                .size(14)
                .symbolic(true);
            
            let content = cosmic::widget::row()
                .push(icon)
                .push(cosmic::widget::text(self.format_temperature(self.temperature)))
                .spacing(4)
                .padding([3, 0, 0, 0]);

            let button = cosmic::widget::button::custom(content)
                .class(cosmic::theme::Button::AppletIcon)
                .on_press(Message::TogglePopup);

            cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
        }
    }
}
