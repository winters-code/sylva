
use glfw;
use glfw::fail_on_errors;

use super::super::dc::window_settings::WindowSettings;
use super::display_manager::DisplayManager;

pub struct WindowHandler<'a> {
    display_handler: DisplayManager<'a>
}

impl<'a> WindowHandler<'a> {
    pub fn new(settings: WindowSettings<'a>) -> WindowHandler<'a> {
        let _display_handler = DisplayManager::new(settings);
        WindowHandler {
            display_handler: _display_handler
        }
    }

    pub fn display_handler(&self) -> &DisplayManager {
        &self.display_handler
    }
}