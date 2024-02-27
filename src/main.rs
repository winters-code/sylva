
// Mark all needed modules as public
pub mod core;
pub mod engine;

// Use the prelude for starting up the engine
use engine::prelude::{WindowSettings, start_engine};

// Run the code
fn main() {

    // Start the engine with the settings
    start_engine(Some(WindowSettings::new((1280 as u32, 960 as u32), Some("Sylva v0.00.0"))));
}