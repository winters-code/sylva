
// Use the window handling modules
pub use crate::core::dc::window_settings::WindowSettings;
use crate::core::window::window_handler::WindowHandler;

// Pollster is used for asynchronous things
use pollster;

// Function to start up the engine
pub fn start_engine(settings: Option<WindowSettings>) {

    // Load the settings
    let _settings = if let Some(s) = settings { s } else { WindowSettings::new((640 as u32, 480 as u32), None) };

    // Create the window
    let mut _window = WindowHandler::new(_settings);

    // Start the window
    pollster::block_on(_window.start());
}