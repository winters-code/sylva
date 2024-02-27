
// GLFW is used to create the windows
use glfw;
use glfw::fail_on_errors;
use glfw::Context;

// Windows setting struct
use super::super::dc::window_settings::WindowSettings;

// Create the DisplayManager struct
pub struct DisplayManager<'a> {

    // GLFW structs
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    // The settings for the window
    settings: WindowSettings<'a>
}

// Implement all the functions for the display manager
#[allow(dead_code)]
impl<'a> DisplayManager<'a> {

    // Create a new DisplayManager
    pub fn new(settings: WindowSettings<'a>) -> DisplayManager<'a> {

        // Load all GLFW objects
        let mut _glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut _window, _events) = _glfw.create_window(settings.size().0, settings.size().1, settings.title(), glfw::WindowMode::Windowed)
            .expect("GLFW window failed to initialized.");

        // Create & return a new DisplayManager
        DisplayManager {
            glfw: _glfw,
            window: _window,
            events: _events,

            settings: settings
        }
    }

    // Start the render loop
    // TODO Change this to run in the WindowHandler instead of here
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

    // Getter for the window settings
    pub fn settings(&self) -> &WindowSettings {
        &self.settings
    }
}