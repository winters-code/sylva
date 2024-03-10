
extern crate glfw;

use super::super::window::display_manager::{GLFWEvents};

pub type Key = glfw::Key;

pub struct InputHandler<'a> {
    _events: &'a GLFWEvents,
    _glfw: &'a glfw::Glfw
}

impl <'a> InputHandler<'a> {
    pub fn new(_events: &'a GLFWEvents, _glfw: &'a glfw::Glfw) -> Self {
        Self {
            _events: _events,
            _glfw: _glfw
        }
    }
}
