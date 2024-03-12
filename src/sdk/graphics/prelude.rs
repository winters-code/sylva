
use super::window::Window;
use super::input::Input;
pub use super::input::{InputState};

pub fn create_window() -> Window {
    Window::new()
}

pub fn create_input() -> Input {
    Input::new()
}