
extern crate glfw;

use glfw::fail_on_errors;

use glfw::{Context};
use super::super::traits::{WindowOwner};

pub struct DisplayHandler {
    _glfw_window: glfw::PWindow,
    _glfw_events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    _glfw: glfw::Glfw
}

impl WindowOwner for DisplayHandler {
    fn is_open(&self) -> bool {
        !self._glfw_window.should_close()
    }

    fn close(&mut self) {
        self._glfw_window.set_should_close(true);
    }
}

impl DisplayHandler {
    pub fn new() -> Self {

        let mut _glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut _glfw_window, _glfw_events) = _glfw.create_window(800, 800, "Test", glfw::WindowMode::Windowed)
            .expect("GLFW failed to initialized");

        Self {
            _glfw: _glfw,
            _glfw_window: _glfw_window,
            _glfw_events: _glfw_events
        }
    }

    pub fn glfw(&mut self) -> &mut glfw::Glfw {
        &mut self._glfw
    }
    pub fn events(&self) -> &glfw::GlfwReceiver<(f64, glfw::WindowEvent)> {
        &self._glfw_events
    }

    pub fn init(&mut self) {
        self._glfw_window.make_current();
        self._glfw_window.set_key_polling(true);
    }

    pub fn update(&mut self) {
        self._glfw_window.swap_buffers();
    }
}