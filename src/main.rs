
#[allow(warnings)]

pub mod backend;
pub mod engine;

use backend::prelude::*;

fn main() {
    let (_window, _input) = make_window((640, 480));
}
