
use super::display_handler::{DisplayHandler};
use super::input_handler::{InputHandler};

pub struct WindowManager {
    _display_handler: DisplayHandler
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            _display_handler: DisplayHandler::new()
        }
    }

    pub fn init(&mut self) -> InputHandler {
        self._display_handler.init();
        InputHandler::new()
    }

    pub fn is_open(&self) -> bool {
        return self._display_handler.is_open();
    }

    pub fn update(&mut self, _input: &InputHandler) {
        _input.update(&mut self._display_handler);
        self._display_handler.update(_input);
    }
}