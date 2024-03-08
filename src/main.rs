
// Allow dead code, for clean logs
#[allow(dead_code, unused_imports)]

// Mark all needed modules as public
pub mod backend;

// Required external modules
use pollster;

// Run the code
fn main() {
    pollster::block_on(backend::start());
}