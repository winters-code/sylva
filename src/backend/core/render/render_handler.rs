
use super::super::setup::window_manager::WindowManager;

pub struct RenderHandler<'a> {
    _surface: wgpu::Surface<'a>,
    _device: wgpu::Device,
    _queue: wgpu::Queue,
    _config: wgpu::SurfaceConfiguration,
    _size: (u32, u32)
}

impl <'a> RenderHandler<'a> {
    pub async fn new(window: &WindowManager) -> Self {
        todo!()
    }
}