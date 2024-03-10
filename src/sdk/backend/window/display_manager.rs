
extern crate glfw;

use glfw::{Action, Key, Context, fail_on_errors};
use super::WindowLike;

pub type GLFWEvents = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

pub struct DisplayManager {
    _glfw: glfw::Glfw,
    _window: glfw::PWindow,
    _events: GLFWEvents
}

impl WindowLike for DisplayManager {
    fn should_close(&self) -> bool {
        self._window.should_close()
    }

    fn close(&self) {
        self._window,set_should_close(true);
    }
}

impl DisplayManager {
    pub fn new() -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw.create_window(300, 300, "Test", glfw::WindowMode::Windowed)
            .expect("GLFW failed to make a window");

        Self {
            _glfw: glfw,
            _window: window,
            _events: events
        }
    }

    pub fn init(&mut self) {
        self._window.make_current();
        self._window.set_key_polling(true);
    }

    pub fn get_events(&self) -> &GLFWEvents {
        &self._events
    }
    pub fn get_glfw(&self) -> &glfw::Glfw {
        &self._glfw
    }
}