
use super::window::Window;
use super::input::Input;
pub use super::input::{InputState};
use super::window_settings::WindowSettings;

pub fn create_window(_s: WindowSettings) -> Window {
    Window::new(_s)
}

pub fn create_input() -> Input {
    Input::new()
}