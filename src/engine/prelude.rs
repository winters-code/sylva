
use super::core::dc::window_settings::WindowSettings;
use super::core::window::window_handler::WindowHandler;
use pollster;

pub fn start_engine() {
    let _settings = WindowSettings::new((640 as u32, 480 as u32), None);
    let mut _window = WindowHandler::new(_settings);
    pollster::block_on(_window.start());
}