
#![allow(warnings)]
// Mark all needed modules as public
pub mod backend;

use pollster;

// Run the code
fn main() {
    pollster::block_on(backend::start());
}