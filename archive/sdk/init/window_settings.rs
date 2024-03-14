
// WindowSettings base struct
pub struct WindowSettings {
    _size: (u32, u32),
    _title: String
}

// Implement the basic functions
impl WindowSettings {

    // Create a new WindowSettings instance with a size and a title
    pub fn new(_size: (u32, u32), _title: String) -> Self {
        Self {
            _size,
            _title
        }
    }
}