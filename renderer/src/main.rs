use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{self, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {ref event,window_id,} => match event {
            WindowEvent::CloseRequested => control_flow.exit(),
            _ => {}
        },
        _ => {}
    }).unwrap();
}

struct State<'a>{
	surface: wgpu::Surface<'a>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	// The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,
}
