
// Allow dead code, for clean logs
#[allow(dead_code, unused_imports)]

// Load the main module
pub mod sdk;

// Required external modules
use pollster;
use sdk::prelude::*;

// Run the code
fn main() {
    pollster::block_on(start_engine());
}