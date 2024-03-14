
extern crate glfw;

use glfw::{Context, fail_on_errors};

pub type Glfw = glfw::Glfw;
pub type GlfwWindow = glfw::PWindow;
pub type GlfwEvents = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

pub struct Display {
    _glfw_window: GlfwWindow,
    _glfw_glfw: Glfw,
    _glfw_events: GlfwEvents
}

impl Display {
    pub fn new() -> Self {

        let mut _glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut _window, _events) = _glfw.create_window(300, 300, "Test", glfw::WindowMode::Windowed).expect("Failed to create GLFW window!");

        Self {
            _glfw_window: _window,
            _glfw_glfw: _glfw,
            _glfw_events: _events
        }
    }

    pub fn init(&mut self) {
        self._glfw_window.make_current();
        self._glfw_window.set_key_polling(true);
    }

    pub fn render(&mut self) {
        self._glfw_window.swap_buffers();
    }

    pub fn should_close(&self) -> bool {
        self._glfw_window.should_close()
    }
    pub fn close(&mut self) {
        self._glfw_window.set_should_close(true);
    }

    pub fn get_events(&self) -> &GlfwEvents { &self._glfw_events }
    
    pub fn get_glfw(&self) -> &Glfw { &self._glfw_glfw }
    pub fn get_glfw_mut(&mut self) -> &mut Glfw { &mut self._glfw_glfw }

}