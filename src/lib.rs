pub mod applet;
pub mod config;
pub mod weather;
pub mod geolocation;

pub fn run() -> cosmic::iced::Result {
    applet::run()
}
