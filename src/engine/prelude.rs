
use super::core::dc::window_settings::WindowSettings;
use super::core::window::window_handler::WindowHandler;

pub fn start_engine() {
    let _settings = WindowSettings::new((640 as u32, 480 as u32), None);
    let _window = WindowHandler::new(_settings);
    println!("{}", &_window.display_handler().settings().size().0);
}