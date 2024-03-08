
use super::display_manager::DisplayManager;

pub struct WindowHandler {
    _display_manager: DisplayManager
}

impl WindowHandler {
    pub fn new() -> Self {
        Self {
            _display_manager: DisplayManager::new()
        }
    }
}