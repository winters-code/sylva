
// Allow dead code, for clean logs
#[allow(warnings)]

// Load the main module
pub mod sdk;

// Required external modules
use pollster;
use sdk::prelude::*;

// Run the code
fn main() {
    pollster::block_on(
        start_engine(
            WindowSettings::new(
                (640u32, 480u32),
                String::from("Window Title")
            )
        )
    );
}