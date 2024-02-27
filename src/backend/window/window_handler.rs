
use super::display_handler::DisplayHandler;
use super::render_handler::RenderHandler;

pub struct WindowHandler {
    display_handler: DisplayHandler,
    render_handler: RenderHandler
}

impl WindowHandler {

    pub fn new(display_handler: DisplayHandler, render_handler: RenderHandler) -> WindowHandler {
        WindowHandler {
            display_handler: display_handler,
            render_handler: render_handler
        }
    }

    pub fn display_handler(&self) -> &DisplayHandler {
        &self.display_handler
    }
    pub fn render_handler(&self) -> &RenderHandler {
        &self.render_handler
    }

}