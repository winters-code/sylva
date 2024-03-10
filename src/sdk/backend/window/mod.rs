pub mod display_manager;
pub mod window_handler;

pub trait WindowLike {
    fn should_close(&self) -> bool {
        false
    }

    fn close(&self) {
        todo!()
    }
}