
use super::display_handler::{DisplayHandler};
use super::input_handler::{InputHandler, InputState};

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

    pub fn update(&mut self, _input: &mut InputHandler) {
        _input.update(&mut self._display_handler);
        self._display_handler.update(_input);

        if let InputState::Down | InputState::JustPressed = _input.get_event("q") {
            self._display_handler.close();
        }
    }
}