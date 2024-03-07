
extern crate glfw;

use std::collections::{HashMap};
use super::display_handler::{DisplayHandler};
use glfw::{Action, Key};

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

    fn set_key(&mut self, key: &str, state: InputState) {
        if let Some(x) = key.get_name() {
            self._input_states.get_mut()
        } else {
            warn!("Key doesn't have name!");
        }
    }

    fn handle_event(&self, e: glfw::WindowEvent) {
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

    pub fn update(&self, display_handler: &mut DisplayHandler) {
        display_handler.glfw().poll_events();

        for (_, event) in glfw::flush_messages(display_handler.events()) {
            println!("{:?}", event);
            self.handle_event(event);
        }
    }

    pub fn get_event(&self, key: &str) -> InputState {
        InputState::Up
    }
}