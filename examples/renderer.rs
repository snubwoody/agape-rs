use helium_renderer::Renderer;
use winit::{
    event::WindowEvent, event_loop::EventLoop, window::WindowBuilder
};

#[tokio::main]
async fn main(){
	let event_loop = EventLoop::new().unwrap();

	let window = WindowBuilder::new()
		.with_visible(false)
		.build(&event_loop)
		.unwrap();

	let renderer = Renderer::new(&window).await;

	event_loop
		.run(|event, window_target| match event {
			winit::event::Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => {
					window_target.exit();
				},
				event => {

				}
			},
			_ => {}
		})
		.expect("Event loop error occured");

}