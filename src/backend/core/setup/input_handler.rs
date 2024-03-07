
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

    fn set_key(&self, key: &str, state: InputState) {
        todo!()
    }

    fn handle_event(&self, e: glfw::WindowEvent) {
        match e {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.set_key("esc", InputState::JustPressed);
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