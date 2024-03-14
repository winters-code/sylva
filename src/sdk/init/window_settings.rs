pub struct WindowSettings {
    _size: (u32, u32),
    _title: String
}

impl WindowSettings {
    pub fn new(_size: (u32, u32), _title: String) -> Self {
        Self {
            _size,
            _title
        }
    }
}