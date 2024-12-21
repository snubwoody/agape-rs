use winit::window::Window;

use crate::widgets::{Widget, WidgetBody};
use super::{events::{ EventQueue}, AppState};

/// A page
pub struct View{
	root_widget:Box<dyn Widget>,
	root_body:WidgetBody,
	event_queue:EventQueue,
}

impl View {
	pub fn new(root_widget:impl Widget + 'static,event_queue:EventQueue) -> Self {
		Self { 
			root_body:root_widget.build(),
			root_widget:Box::new(root_widget),
			event_queue
		}
	}

	pub fn handle_events(&mut self,event: winit::event::WindowEvent,window:&Window){
		// Pass the events to the event manager to determine which events have fired
		// for which widgets.
		self.event_queue.handle_events(&event,&self.root_body); // TODO should return vec instead
		window.request_redraw();
	}
	
	pub fn render(&mut self,state:&AppState) {
		let output = state.surface.get_current_texture().unwrap(); // TODO maybe handle this error
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		
		let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
			label:Some("Render encoder")
		});
		
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color{r: 1.0,g: 1.0,b: 1.0,a:1.0}),
					store: wgpu::StoreOp::Store, // TODO discard might potentially be faster
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});

		self.root_body.arrange(state.size);
		self.root_body.render(&mut render_pass,state);
		// Rebuild the widgets every frame
		//self.root_widget.build().render(&mut render_pass,&state);
		
		// Drop the render pass because it borrows encoder
		// mutably
		std::mem::drop(render_pass);
	
		state.queue.submit(std::iter::once(encoder.finish()));
		output.present();
	}
}




