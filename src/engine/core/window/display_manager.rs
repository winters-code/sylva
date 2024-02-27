
use super::super::dc::window_settings::WindowSettings;

pub struct DisplayManager<'a> {
    settings: WindowSettings<'a>
}

impl<'a> DisplayManager<'a> {
    pub fn new(settings: WindowSettings<'a>) -> DisplayManager<'a> {
        DisplayManager {
            settings: settings
        }
    }

    pub fn settings(&self) -> &WindowSettings {
        &self.settings
    }
}