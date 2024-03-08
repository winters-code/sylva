
// Import the required handlers to make this work
use super::display_handler::{DisplayHandler};
use super::input_handler::{InputHandler, InputState};
use super::super::traits::{WindowOwner};

// Create the window manager
pub struct WindowManager {
    _display_handler: DisplayHandler
}

impl WindowOwner for WindowManager {

    // Check if the window is open or not
    fn is_open(&self) -> bool {
        return self._display_handler.is_open();
    }

    // Close it
    fn close(&mut self) {
        self._display_handler.close();
    }

}

// Load all the needed functions for the window manager
impl WindowManager {

    // Create a new window manager instance
    pub fn new() -> Self {
        Self {
            _display_handler: DisplayHandler::new()
        }
    }

    // Initialize the window
    pub fn init(&mut self) -> InputHandler {
        self._display_handler.init();

        // Also create a new input handler
        InputHandler::new()
    }

    // Update the window
    pub fn update(&mut self, _input: &mut InputHandler) {

        // Handle input
        _input.update(&mut self._display_handler);

        // Update the display
        self._display_handler.update();
    }
}