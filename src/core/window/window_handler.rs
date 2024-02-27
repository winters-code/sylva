
use super::super::dc::window_settings::WindowSettings;
use super::display_manager::DisplayManager;

pub struct WindowHandler<'a> {
    display_manager: DisplayManager<'a>
}

#[allow(dead_code)]
impl<'a> WindowHandler<'a> {
    pub fn new(settings: WindowSettings<'a>) -> WindowHandler<'a> {
        let mut _display_manager = DisplayManager::new(settings);
        WindowHandler {
            display_manager: _display_manager
        }
    }

    pub async fn start(&mut self) {
        println!("Window has been run.");
        self.display_manager.start();
    }

    pub fn display_manager(&self) -> &DisplayManager {
        &self.display_manager
    }
}