pub(in super) mod core;

use core::setup::{WindowManager};

pub async fn start() {
    let mut _window_manager = WindowManager::new();
    let mut _input_handler = _window_manager.init();

    while _window_manager.is_open() {
        _window_manager.update(&mut _input_handler);
    }
}