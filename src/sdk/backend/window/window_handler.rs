
extern crate glfw;

use super::display_manager::DisplayManager;
use super::super::input::input_handler::InputHandler;
use super::WindowLike;

pub struct WindowHandler {
    _display_manager: DisplayManager
}

impl WindowLike for WindowHandler {
    fn should_close(&self) -> bool {
        self._display_manager.should_close()
    }

    fn close(&self) {
        self._display_manager.close();
    }
}

impl WindowHandler {
    pub fn new() -> Self {
        Self {
            _display_manager: DisplayManager::new()
        }
    }

    pub fn init(&mut self) -> InputHandler {
        self._display_manager.init();
        InputHandler::new(self._display_manager.get_events(), self._display_manager.get_glfw())
    }
}