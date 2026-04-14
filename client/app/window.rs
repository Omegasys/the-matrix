use winit::{
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: WinitWindow,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Failed to create window");

        Self { event_loop, window }
    }
}
