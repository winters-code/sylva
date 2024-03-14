
// Load GLFW for window handling
extern crate glfw;

// Base GLFW stuff needed to handle a window
use glfw::{Context, fail_on_errors};

// Simple type aliases to make it easier to write everything cleanly
pub type Glfw = glfw::Glfw;
pub type GlfwWindow = glfw::PWindow;
pub type GlfwEvents = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

// Create the display struct
pub struct Display {
    _glfw_window: GlfwWindow,
    _glfw_glfw: Glfw,
    _glfw_events: GlfwEvents
}

// Implement all the functiosn for the display struct
impl Display {

    // Create a new display struct
    pub fn new() -> Self {

        // Load all the GLFW objects
        let mut _glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut _window, _events) = _glfw.create_window(300, 300, "Test", glfw::WindowMode::Windowed).expect("Failed to create GLFW window!");

        // Create and return the new Display instance
        Self {
            _glfw_window: _window,
            _glfw_glfw: _glfw,
            _glfw_events: _events
        }
    }

    // Initialize the display
    pub fn init(&mut self) {

        // Make the window the currently active one
        self._glfw_window.make_current();

        // Make the window start key polling
        self._glfw_window.set_key_polling(true);
    }

    // Render the window
    pub fn render(&mut self) {

        // Swap the front and back buffers
        self._glfw_window.swap_buffers();
    }

    // Wether or not the window should close
    pub fn should_close(&self) -> bool {
        self._glfw_window.should_close()
    }

    // Force the window to close
    pub fn close(&mut self) {
        self._glfw_window.set_should_close(true);
    }

    // Getters, self explainatory
    pub fn get_events(&self) -> &GlfwEvents { &self._glfw_events }
    pub fn get_glfw(&self) -> &Glfw { &self._glfw_glfw }
    pub fn get_glfw_mut(&mut self) -> &mut Glfw { &mut self._glfw_glfw }
}