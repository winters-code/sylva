
use super::display::Display;
use super::input::Input;

pub struct Window {
    _display: Display
}

impl Window {
    pub fn new() -> Self {
        Self {
            _display: Display::new()
        }
    }

    pub fn update(&mut self, _input: &mut Input) {
        _input.update(self.get_display_mut());
    }

    pub fn init(&mut self) {
        self._display.init();
    }

    pub fn render(&mut self) {
        self._display.render();
    }

    pub fn should_close(&self) -> bool {
        self._display.should_close()
    }
    pub fn close(&mut self) {
        self._display.close();
    }

    pub fn get_display(&self) -> &Display {
        &self._display
    }
    pub fn get_display_mut(&mut self) -> &mut Display {
        &mut self._display
    }
}