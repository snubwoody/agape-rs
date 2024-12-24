use std::collections::HashMap;

use crystal::{Layout, LayoutSolver};
use helium_core::{position::Position, size::Size};
use winit::window::Window;

use crate::widgets::{Widget, WidgetBody};
use super::{events::{ EventQueue}, AppState};

/// A page
pub struct View{
	root_layout:Box<dyn crystal::Layout>,
	root_widget:Box<dyn Widget>,
	root_body:WidgetBody,
	event_queue:EventQueue,
	layout_map:HashMap<String,(Size,Position)>
}

impl View {
	pub fn new(root_widget:impl Widget + 'static,event_queue:EventQueue) -> Self {
		let (root_body,root_layout) = root_widget.build();
		
		Self { 
			root_body,
			root_layout,
			root_widget:Box::new(root_widget),
			event_queue,
			layout_map:HashMap::new()
		}
	}

	pub fn handle_events(&mut self,event: winit::event::WindowEvent,window:&Window){
		// Pass the events to the event manager to determine which events have fired
		// for which widgets.
		self.event_queue.handle_events(&event,&self.root_body);
		window.request_redraw();
	}

	pub fn build_layout_map(&mut self){
		self.layout_map.insert(
			self.root_layout.id().to_string(), 
			(
				self.root_layout.size(),
				self.root_layout.position(),
			)
		);
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

		LayoutSolver::solve(&mut *self.root_layout, state.size);
		self.build_layout_map();

		//dbg!(&self.root_layout);
		//dbg!(&self.root_layout);
		self.root_body.update_sizes(&self.root_layout);
		self.root_body.render(&mut render_pass,state);
		//dbg!(&self.root_body);		
		
		// Drop the render pass because it borrows encoder
		// mutably
		std::mem::drop(render_pass);
	
		state.queue.submit(std::iter::once(encoder.finish()));
		output.present();
	}
}




