
pub struct WindowSettings<'a>  {
    size: (u32, u32),
    title: &'a str
}

impl <'a> WindowSettings<'a> {
    pub fn new(size: (u32, u32), title: Option<&'a str>) -> WindowSettings<'a> {
        WindowSettings {
            size: size,
            title: if let Some(x) = title { x } else { "Unnamed Window" }
        }
    }

    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }
    pub fn title(&self) -> &'a str {
        &self.title
    }
}