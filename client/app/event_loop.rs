use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

pub struct AppEventLoop;

impl AppEventLoop {
    pub fn run<F>(event_loop: winit::event_loop::EventLoop<()>, mut update: F)
    where
        F: 'static + FnMut(),
    {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    update();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
