
// Load all the required modules
use super::super::dc::window_settings::WindowSettings;
use super::display_manager::DisplayManager;

// Create the base struct for window handling
pub struct WindowHandler<'a> {
    display_manager: DisplayManager<'a>
}

// Implement all the functions
#[allow(dead_code)]
impl<'a> WindowHandler<'a> {

    // Create a new WindowHanler
    pub fn new(settings: WindowSettings<'a>) -> WindowHandler<'a> {

        // Create the DisplayManager
        let mut _display_manager = DisplayManager::new(settings);

        // Create and return a new WindowHandler
        WindowHandler {
            display_manager: _display_manager
        }
    }

    // Start the window
    pub async fn start(&mut self) {
        println!("Window has been run.");
        self.display_manager.start();
    }

    // Getter for the DisplayManager
    pub fn display_manager(&self) -> &DisplayManager {
        &self.display_manager
    }
}