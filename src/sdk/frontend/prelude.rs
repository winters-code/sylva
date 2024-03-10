
use crate::sdk::backend::window::{
    WindowLike,
    window_handler::WindowHandler
};

pub async fn start_engine() {
    let mut _w = WindowHandler::new();
    let _i = _w.init();

    while !_w.should_close() {
        _w.close();
    }
}