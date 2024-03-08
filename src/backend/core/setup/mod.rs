
// Mark all these modules as public
pub mod display_handler;
pub mod window_manager;
pub mod input_handler;

// Everything required for a minimal window manager
pub use window_manager::WindowManager;
pub use input_handler::InputState;
