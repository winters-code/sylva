
extern crate glfw;

use std::collections::{HashMap};
use super::display_handler::{DisplayHandler};
use glfw::{Action, Key};

#[derive(Clone)]
pub enum InputState {
    Up,
    Down,
    JustPressed,
    JustReleased
}

pub struct InputHandler {
    _input_states: HashMap<String, InputState>
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            _input_states: HashMap::new()
        }
    }

    fn set_key(&mut self, key: Option<String>, state: InputState) {
        if let Some(x) = key {
            let mut key_index = self._input_states.get_mut(&x);
            if let Some(y) = key_index {
                *y = state;
            } else {
                self._input_states.insert(x, state);
            }
        } else {
            println!("Key doesn't have name! key: {:?}", key);
        }
    }

    fn handle_event(&mut self, e: glfw::WindowEvent) {
        match e {
            glfw::WindowEvent::Key(key, _, action, _) => {
                let input_state = if action == Action::Press { InputState::JustPressed }
                                else if action == Action::Repeat { InputState::Down }
                                else { InputState::JustReleased };
                self.set_key(key.get_name(), input_state);
            },
            _ => {}
        }
    }

    pub fn update(&mut self, display_handler: &mut DisplayHandler) {
        self._input_states.clear();
        display_handler.glfw().poll_events();

        for (_, event) in glfw::flush_messages(display_handler.events()) {
            self.handle_event(event);
        }
    }

    pub fn get_event(&self, key: &str) -> InputState {
        self._input_states.get(key).unwrap_or(&InputState::Up).clone()
    }
}