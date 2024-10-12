use glium::{
	glutin::surface::WindowSurface,
	Display,
	Surface,
	Program,
};
use winit::window::Window;
use crate::widgets::{Widget, WidgetTree};
use super::RenderContext;

/// A page
pub struct View{
	pub widget_tree:WidgetTree
}

impl View {
	pub fn new(root_widget:impl Widget + 'static) -> Self {
		let widget_tree = WidgetTree::new(root_widget.build());		
		Self { widget_tree }
	}
	
	pub fn render(
		&mut self,
		context:&RenderContext,
		device: &wgpu::Device,
		surface: &wgpu::Surface,
		queue: &wgpu::Queue,
		window:&Window
	) {
		/* // Create a frame that will be drawn to
		let mut frame = display.draw();
		frame.clear_color(1.0, 1.0, 1.0, 1.0);

		//Render the widget tree
		self.widget_tree.render(display,&mut frame,window,context);

		//Swap the buffers
		frame.finish().unwrap(); */

		let output = surface.get_current_texture().unwrap();
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
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

		self.widget_tree.render(window, context, &render_pass);

		// Drop the render pass because it borrows encoder
		// mutably
		drop(render_pass);
	
		queue.submit(std::iter::once(encoder.finish()));
		output.present();

	}
}

/* // TODO try fitting the window and display in the render context
/// Contains the compiled shader programs
#[derive(Debug)]
pub struct RenderContext{
	pub surface_program:Program,
	pub text_program:Program,
	pub image_program:Program
}

impl RenderContext {
	// TODO change this to use the from source method of the Program struct
	pub fn new(
		surface_program:Program,
		text_program:Program,
		image_program:Program
	) -> Self {
		Self{ 
			surface_program, 
			text_program,
			image_program
		}
	}
}

 */




