
// Allow dead code, for clean logs
#[allow(dead_code, unused_imports)]

// Load all the modules
pub mod engine;

// Required external modules
use pollster;
use engine::prelude::start_engine;

// Run the code
fn main() {
    pollster::block_on(start_engine());
}