
use glfw;
use glfw::fail_on_errors;
use glfw::Context;

use super::super::dc::window_settings::WindowSettings;

pub struct DisplayManager<'a> {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    settings: WindowSettings<'a>
}

impl<'a> DisplayManager<'a> {
    pub fn new(settings: WindowSettings<'a>) -> DisplayManager<'a> {
        let mut _glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut _window, _events) = _glfw.create_window(settings.size().0, settings.size().1, settings.title(), glfw::WindowMode::Windowed)
            .expect("GLFW window failed to initialized.");

        DisplayManager {
            glfw: _glfw,
            window: _window,
            events: _events,

            settings: settings
        }
    }

    pub fn start(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);

        while !self.window.should_close() {
            self.window.swap_buffers();

            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&(self.events)) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }

    pub fn settings(&self) -> &WindowSettings {
        &self.settings
    }
}