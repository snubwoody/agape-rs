
use crate::widgets::{Widget, WidgetTree};
use super::{AppState};

/// A page
pub struct View{
	pub widget_tree:WidgetTree
}

impl View {
	pub fn new(root_widget:impl Widget + 'static) -> Self {
		let widget_tree = WidgetTree::new(root_widget.build());		
		Self { widget_tree }
	}
	
	pub fn render(&mut self,state:&AppState) {		
		let output = state.surface.get_current_texture().unwrap();
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
					load: wgpu::LoadOp::Clear(wgpu::Color {
						r: 1.0,
						g: 1.0,
						b: 1.0,
						a: 1.0,
					}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});

		self.widget_tree.render(&state.size, &state.context, &mut render_pass,&state.device);

		// Drop the render pass because it borrows encoder
		// mutably
		drop(render_pass);
	
		state.queue.submit(std::iter::once(encoder.finish()));
		output.present();

	}
}




