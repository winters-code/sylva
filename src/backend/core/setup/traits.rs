
pub trait WindowOwner {
    fn is_open(&self) -> bool;

    fn close(&mut self);
}