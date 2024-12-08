use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor};
use crate::{
	app::{AppState, RenderContext}, impl_surface, surface::Surface, vertex::Vertex, Bounds, Color, Position, Size
};

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the color, coordinates and size.
#[derive(Debug,Clone,PartialEq,Default)]
pub struct CircleSurface{
	pub position:Position,
	pub size:Size,
	pub color:Color,
}

impl CircleSurface {
	pub fn new(radius:u32,color:Color) -> Self{
		dbg!(radius);
		let size = Size::new(200.0, 200.0);
		let position = Position::default();
		dbg!(&size);
		Self { position,size,color }
	}

	pub fn color(&mut self,color:Color) {
		self.color = color
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let color = self.color.normalize();
		dbg!(&self);
		let x = self.position.x;
		let y = self.position.y;

		let vertex1 = Vertex::new(x, y,color); //Top left
		let vertex2 = Vertex::new(x+self.size.width, y,color); // Top right
		let vertex3 = Vertex::new(x, y+self.size.height,color); //Bottom left
		let vertex4 = Vertex::new(x+self.size.width, y,color); //Top right
		let vertex5 = Vertex::new(x, y+self.size.height,color); // Bottom left
		let vertex6 = Vertex::new(x+self.size.width, y+self.size.height,color); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

}

impl Surface for CircleSurface {
	fn draw(
		&self,
		render_pass:&mut wgpu::RenderPass,
		context: &RenderContext,
		state: &AppState
	) {
		let vertices = self.to_vertices();
		dbg!(&vertices);
		
		let vertex_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let size_buffer = state.device.create_buffer_init(
			&BufferInitDescriptor{
				label:Some("Size buffer"),
				contents: bytemuck::cast_slice(&[self.size.width,self.size.height]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}
		);

		let position_buffer = state.device.create_buffer_init(
			&BufferInitDescriptor{
				label:Some("Position buffer"),
				contents: bytemuck::cast_slice(&[self.position.x,self.position.y]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}
		);

		let bound_bind_group = state.device.create_bind_group(
			&BindGroupDescriptor{
				label:Some("Cirlce bounds bind group"),
				layout:&context.circle_renderer.bounds_layout,
				entries:&[
					wgpu::BindGroupEntry{
						binding:0,
						resource:size_buffer.as_entire_binding()
					},
					wgpu::BindGroupEntry{
						binding:1,
						resource:position_buffer.as_entire_binding()
					}
				]
			}
		);

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&context.circle_renderer.render_pipeline);
		render_pass.set_bind_group(0, &context.circle_renderer.window_bind_group, &[]);
		render_pass.set_bind_group(1, &bound_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

		render_pass.draw(0..vertices.len() as u32, 0..1);
	}

	impl_surface!();
}
