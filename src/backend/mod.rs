
// Load the core module
pub(in super) mod core;

// Everything needed to start the engine
use core::setup::{WindowManager};
use core::traits::WindowOwner;
use core::setup::InputState;

// Start the engine
pub async fn start() {

    // Create the structs needed
    let mut _window_manager = WindowManager::new();
    let mut _input_handler = _window_manager.init();

    // While the engine is running
    while _window_manager.is_open() {

        // Update the window
        _window_manager.update(&mut _input_handler);
        if let InputState::Down | InputState::JustPressed = _input_handler.get_event("q") {
            _window_manager.close();
        }
    }
}