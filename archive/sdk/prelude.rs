
// Import the requirements for the engine
use crate::sdk::init::prelude::*;

// Allow anyone to use the WindowSettings struct
pub use crate::sdk::init::window_settings::WindowSettings;

// Function to start the engine
pub async fn start_engine(_s: WindowSettings) {

    // Create the window & input systems
    let mut _w = create_window(_s);
    let mut _i = create_input();
    
    // Initialize the window
    _w.init();
    
    // While the window is open
    while !_w.should_close() {

        // Update & render the window
        _w.update(&mut _i);
        _w.render();

        // If you pressed the close key, close the window
        if _i.get_state("escape") == &InputState::JustPressed {
            _w.close();
        }
    }
}