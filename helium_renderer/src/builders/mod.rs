mod buffer;
mod bind_group;
use winit::{platform::windows::EventLoopBuilderExtWindows, window::WindowBuilder};
pub use buffer::*;
pub use bind_group::*;

#[cfg(test)]
pub async fn setup() -> (wgpu::Device,wgpu::Queue){
	let event_loop = winit::event_loop::EventLoopBuilder::new()
		.with_any_thread(true)
		.build().expect("Failed to create EventLoop");

	let window = WindowBuilder::new()
		.with_visible(false)
		.build(&event_loop)
		.expect("Failed to create window");

	// Handle to wpgu for creating a surface and an adapter
	let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
		backends: wgpu::Backends::PRIMARY,
		..Default::default()
	});

	// Create the surface to draw on
	let surface = instance.create_surface(window).unwrap();

	// Handle to the graphics card
	let adapter = instance
		.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: Default::default(),
			compatible_surface: Some(&surface),
			force_fallback_adapter: false,
		})
		.await
		.unwrap();

	// The device is an open connection to the graphics
	// card and the queue is a command buffer
	let (device, queue) = adapter
		.request_device(
			&wgpu::DeviceDescriptor {
				label: Some("Device/Queue"),
				required_features: wgpu::Features::empty(),
				..Default::default()
			},
			None,
		)
		.await
		.unwrap();
	
	(device,queue)
}
