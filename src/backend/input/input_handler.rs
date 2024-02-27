
use std::collections::HashMap;
use crate::backend::window::display_handler::DisplayHandler;

use glfw;

pub enum InputState {
    Press,   // When it's pressed
    Pressed, // When it's held down
    Release, // When it's released
    Released // When it's held up
}

pub struct InputHandler<'core> {
    keys: HashMap<&'static str, InputState>,
    close_requested: bool,

    events: &'core glfw::GlfwReceiver<(f64, glfw::WindowEvent)>
}

impl <'core> InputHandler<'core> {

    pub fn new(events: &'core glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) -> InputHandler<'core> {
        InputHandler {
            keys: HashMap::new(),
            close_requested: false,
            events: events
        }
    }

    pub fn should_close_window(&self) -> bool {
        self.close_requested
    }
    pub fn get_state(&self, key: &'static str) -> InputState {
        todo!()
    }

    fn handle_event(&self, event: (f64, glfw::WindowEvent)) {
        todo!()
    }

    pub fn poll_events(&self) {
        todo!();
    }

}