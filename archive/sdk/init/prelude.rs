
// Load all the needed libraries
use super::window::Window;
use super::input::Input;
use super::window_settings::WindowSettings;

// Import InputState for the user to use
pub use super::input::{InputState};

// Create a new, empty window
pub fn create_window(_s: WindowSettings) -> Window {
    Window::new(_s)
}

// Create a new, empty input handler
pub fn create_input() -> Input {
    Input::new()
}