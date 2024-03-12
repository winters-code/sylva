
extern crate glfw;

use super::display::Display;

pub struct Input {
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
        }
    }

    pub fn update(&mut self, _d: &mut Display) {
        _d.get_glfw_mut().poll_events();

        for (_, e) in glfw::flush_messages(_d.get_events()) {
            match e {
                glfw::WindowEvent::Key(k, sc, state, _) => {
                    if k as i32 != -1 {
                        println!("{:?}", get_key_name(k));
                    }
                },
                _ => {}
            }
        }
    }
}