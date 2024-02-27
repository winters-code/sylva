
use super::core::dc::window_settings::WindowSettings;

pub fn start_engine() {
    let _settings = WindowSettings::new((640 as u32, 480 as u32), None);
    println!("{}", _settings.size().0);
}