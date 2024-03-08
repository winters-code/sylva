
pub trait WindowOwner {
    fn is_open(&self) -> bool {
        false
    }

    fn close(&mut self) {
        println!("Undefined close method");
    }
}