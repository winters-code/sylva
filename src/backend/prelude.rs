
use super::window::window_handler::WindowHandler;
use super::window::display_handler::DisplayHandler;
use super::window::render_handler::RenderHandler;
use super::input::input_handler::InputHandler;

pub fn make_window(window_size: (u32, u32)) -> (WindowHandler, InputHandler<'static>) {
    let dh = DisplayHandler::new(window_size);
    let rh = RenderHandler::new();
    let wh = WindowHandler::new(dh, rh);
    (wh, InputHandler::new((&wh).display_handler().events()))
}