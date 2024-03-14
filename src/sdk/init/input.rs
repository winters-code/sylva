
// Load GLFW for the input stuff
extern crate glfw;

// Use all the required libraries + files
use super::display::Display;
use std::collections::HashMap;

// Create the input state enum
#[derive(Debug, PartialEq)]
pub enum InputState {
    JustPressed,
    JustReleased,
    HeldDown,
    Released
}

// Create the base input structure
pub struct Input {
    _inputs: HashMap<String, InputState>
}

// Get the state from a GLFW action
fn get_state_from(state: glfw::Action) -> InputState {
    match state {
        (glfw::Action::Press) => {InputState::JustPressed},
        (glfw::Action::Release) => {InputState::JustReleased},
        (glfw::Action::Repeat) => {InputState::HeldDown}
    }
}

// Simple type alias for GLFW keys
type k = glfw::Key;

// Get the key name from a key
fn get_key_name(key: k) -> String {

    // If the key exists
    if key != k::Unknown {

        // If the key name exists
        if let Some(_name) = glfw::get_key_name(Some(key), None) {

            //Return the key name
            return _name;
        }
    }

    // If either of the previous ones failed, use this MASSIVE `match` statement to trun them into a string
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

// Implement the functions for the input struct
impl Input {

    // Create a new empty Input struct
    pub fn new() -> Self {
        Self {
            _inputs: HashMap::new()
        }
    }

    // Update the inputs received
    pub fn update(&mut self, _d: &mut Display) {

        // Reset the inputs pressed
        self.reset_input();

        // Poll the events
        _d.get_glfw_mut().poll_events();

        // For each event polled, handle it
        for (_, e) in glfw::flush_messages(_d.get_events()) {
            self.handle_event(e);
        }
    }

    // Reset the nputs pressed
    fn reset_input(&mut self) {

        // For each key input in the _inputs hashmap
        for (k, val) in self._inputs.iter_mut() {

            // Set it to the next state it should be
            match val {
                (InputState::JustPressed) => {*val = InputState::HeldDown},
                (InputState::JustReleased) => {*val = InputState::Released},
                _ => {}
            }
        }
    }

    // Handle a single event
    fn handle_event(&mut self, e: glfw::WindowEvent) {

        // To make sure we're only handling what we can handle
        match e {

            // Handle a key press
            glfw::WindowEvent::Key(k, sc, state, _) => {

                // Get the name and the state of the key, and set the input state.
                let name = get_key_name(k);
                let state = get_state_from(state);
                self.set_input(name, state);
            },

            // To not cause errors for the other possible states
            _ => {}
        }
    }

    // Set the state of a specific input
    fn set_input(&mut self, name: String, state: InputState) {

        // Get the key
        let key = self._inputs.get_mut(&name);

        // If the key exists, set that key in the dictionary to the state
        if let Some(k) = key {
            *k = state;
        } else {
            self._inputs.insert(name, state);
        }
    }

    // Get the state of a key
    pub fn get_state(&self, key: &str) -> &InputState {
        self._inputs.get(&String::from(key)).unwrap_or(&InputState::Released)
    }
}