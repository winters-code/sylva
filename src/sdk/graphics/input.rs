
extern crate glfw;

use super::display::Display;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum InputState {
    JustPressed,
    JustReleased,
    HeldDown,
    Released
}

pub struct Input {
    _inputs: HashMap<String, InputState>
}

pub fn get_state_from(state: glfw::Action) -> InputState {
    match state {
        (glfw::Action::Press) => {InputState::JustPressed},
        (glfw::Action::Release) => {InputState::JustReleased},
        (glfw::Action::Repeat) => {InputState::HeldDown}
    }
}

type k = glfw::Key;
pub fn get_key_name(key: k) -> String {
    if let Some(_name) = glfw::get_key_name(Some(key), None) {
        _name
    } else {
        match key {
            (k::Space) => {String::from("space")},
            (k::LeftSuper) => {String::from("left_meta")},
            (k::RightSuper) => {String::from("right_meta")},
            (k::Escape) => {String::from("escape")},
            (k::Enter) => {String::from("enter")},
            (k::Tab) => {String::from("tab")},
            (k::Backspace) => {String::from("delete")},
            (k::Delete) => {String::from("delete")},
            (k::Up) => {String::from("up")},
            (k::Down) => {String::from("down")},
            (k::Left) => {String::from("left")},
            (k::Right) => {String::from("right")},
            (k::F1) => {String::from("function_1")},
            (k::F2) => {String::from("function_2")},
            (k::F3) => {String::from("function_3")},
            (k::F4) => {String::from("function_4")},
            (k::F5) => {String::from("function_5")},
            (k::F6) => {String::from("function_6")},
            (k::F7) => {String::from("function_7")},
            (k::F8) => {String::from("function_8")},
            (k::F9) => {String::from("function_9")},
            (k::F10) => {String::from("function_10")},
            (k::F11) => {String::from("function_11")},
            (k::F12) => {String::from("function_12")},
            (k::LeftControl) => {String::from("left_control")},
            (k::LeftShift) => {String::from("left_shift")},
            (k::LeftAlt) => {String::from("left_alt")},
            (k::RightControl) => {String::from("right_control")},
            (k::RightShift) => {String::from("right_shift")},
            (k::RightAlt) => {String::from("right_alt")},
            _ => {String::from("undefined")}
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            _inputs: HashMap::new()
        }
    }

    pub fn update(&mut self, _d: &mut Display) {
        self.reset_input();

        _d.get_glfw_mut().poll_events();

        for (_, e) in glfw::flush_messages(_d.get_events()) {
            self.handle_event(e);
        }
    }

    fn reset_input(&mut self) {
        for (k, val) in self._inputs.iter_mut() {
            match val {
                (InputState::JustPressed) => {*val = InputState::HeldDown},
                (InputState::JustReleased) => {*val = InputState::Released},
                _ => {}
            }
        }
    }

    fn handle_event(&mut self, e: glfw::WindowEvent) {
        match e {
            glfw::WindowEvent::Key(k, sc, state, _) => {
                let name = get_key_name(k);
                let state = get_state_from(state);
                self.set_input(name, state);
            },
            _ => {}
        }
    }

    fn set_input(&mut self, name: String, state: InputState) {
        let key = self._inputs.get_mut(&name);
        if let Some(k) = key {
            *k = state;
        } else {
            self._inputs.insert(name, state);
        }
    }

    pub fn get_state(&self, key: &str) -> &InputState {
        self._inputs.get(&String::from(key)).unwrap_or(&InputState::Released)
    }
}