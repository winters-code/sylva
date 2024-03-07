
extern crate glfw;

use std::collections::{HashMap};
use super::display_handler::{DisplayHandler};

pub enum InputState {
    Up,
    Down,
    JustPressed,
    JustReleased
}

pub struct InputHandler {
    _input_states: HashMap<&'static str, InputState>
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            _input_states: HashMap::new()
        }
    }

    pub fn update(&self, display_handler: &mut DisplayHandler) {
        display_handler.glfw().poll_events();

        for (_, event) in glfw::flush_messages(display_handler.events()) {
            println!("{:?}", event);
            match event {
                _ => {}
            }
        }
    }

    pub fn get_event(&self, key: &str) -> InputState {
        InputState::Up
    }
}