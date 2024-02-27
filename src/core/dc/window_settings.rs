
// Create the WindowSettings struct
pub struct WindowSettings<'a>  {
    size: (u32, u32),
    title: &'a str
}

// Implement all the required functions for the window settings
#[allow(dead_code)]
impl <'a> WindowSettings<'a> {

    // Create a new WindowSettings instance
    pub fn new(size: (u32, u32), title: Option<&'a str>) -> WindowSettings<'a> {

        // Create and return a new WindowSettings instance
        WindowSettings {
            size: size,
            title: if let Some(x) = title { x } else { "Unnamed Window" }
        }
    }

    // Getter for the size of the window
    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }

    // Getter for the title of the window
    pub fn title(&self) -> &'a str {
        &self.title
    }
}