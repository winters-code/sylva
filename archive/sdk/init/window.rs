
// Use all of the required libraries
use super::display::Display;
use super::input::Input;
use super::window_settings::WindowSettings;

// Create the base struct layour
pub struct Window {
    _display: Display,
    _settings: WindowSettings
}

// Implement all the Window functions
impl Window {

    // Factory method to make te new window with window settings
    pub fn new(_s: WindowSettings) -> Self {
        Self {
            _display: Display::new(),
            _settings: _s
        }
    }

    // Update the window
    pub fn update(&mut self, _input: &mut Input) {

        // Update the input
        _input.update(self.get_display_mut());
    }

    // Initialize the window
    pub fn init(&mut self) {

        // Initialize the display
        self._display.init();
    }

    // Render the window
    pub fn render(&mut self) {

        // Render the display
        self._display.render();
    }

    // If the window should close
    pub fn should_close(&self) -> bool {
        self._display.should_close()
    }

    // Force the window to close
    pub fn close(&mut self) {
        self._display.close();
    }

    // Getters, self-explainatory
    pub fn get_display(&self) -> &Display { &self._display }
    pub fn get_display_mut(&mut self) -> &mut Display { &mut self._display }
    pub fn get_settings(&self) -> &WindowSettings { &self._settings }
}