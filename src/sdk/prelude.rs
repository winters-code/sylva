
use crate::sdk::init::prelude::*;
pub use crate::sdk::init::window_settings::WindowSettings;

pub async fn start_engine(_s: WindowSettings) {
    let mut _w = create_window(_s);
    let mut _i = create_input();
    
    _w.init();
    
    while !_w.should_close() {
        _w.update(&mut _i);
        _w.render();

        if _i.get_state("escape") == &InputState::JustPressed {
            _w.close();
        }
    }
}