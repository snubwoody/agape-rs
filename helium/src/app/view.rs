use winit::window::Window;

use crate::widgets::{Widget, WidgetBody};
use super::{events::EventHandler, AppState};

/// A page
pub struct View{
	root_widget:Box<dyn Widget>,
	root_body:WidgetBody,
	event_handler:EventHandler
}

impl View {
	pub fn new(root_widget:impl Widget + 'static) -> Self {
		Self { 
			root_body:root_widget.build(),
			root_widget:Box::new(root_widget),
			event_handler: EventHandler::new()
		}
	}

	pub fn handle_events(&mut self,event: winit::event::WindowEvent,window:&Window){
		self.event_handler.handle_events(&event,&mut self.root_widget,&mut self.root_body);
		window.request_redraw();
	}
	
	pub fn render(&mut self,state:&AppState) {
		let output = state.surface.get_current_texture().unwrap();
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
			label:Some("Render encoder")
		});
		
		// TODO maybe creating the render pass every frame is unnecessary
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color{r: 1.0,g: 1.0,b: 1.0,a:1.0}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});

		self.root_widget.build().render(&mut render_pass,&state);

		// Drop the render pass because it borrows encoder
		// mutably
		drop(render_pass);
	
		state.queue.submit(std::iter::once(encoder.finish()));
		output.present();
	}
}




