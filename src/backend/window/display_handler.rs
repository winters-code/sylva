
use glfw;
use glfw::fail_on_errors;

pub struct DisplayHandler {
    display: glfw::PWindow,
    glfw: glfw::Glfw,
    pub(in super::super) events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>
}

impl DisplayHandler {
    pub fn new(window_size: (u32, u32)) -> DisplayHandler {

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut display, events) = glfw.create_window(window_size.0, window_size.1, "Test", glfw::WindowMode::Windowed)
            .expect("GLFW window failed to initialize.");

        DisplayHandler {
            display: display,
            events: events,
            glfw: glfw
        }
    }

    pub fn display(&self) -> &glfw::PWindow {
        &self.display
    }
    pub fn events(&self) -> &glfw::GlfwReceiver<(f64, glfw::WindowEvent)> {
        &self.events
    }
}
