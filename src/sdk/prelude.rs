
use crate::sdk::graphics::prelude::*;

pub async fn start_engine() {
    let mut _w = create_window();
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