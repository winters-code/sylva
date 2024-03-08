
// Mark all these modules as public
pub mod display_handler;
pub mod window_manager;
pub mod input_handler;
pub mod traits;

// Everything required for a minimal window manager
pub use window_manager::WindowManager;
