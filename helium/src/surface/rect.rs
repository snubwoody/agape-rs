use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor};
use crate::{
	app::{AppState, RenderContext}, 
	Color, 
	surface::Surface, 
	{Bounds, Position, Size}, 
	vertex::Vertex
};

// TODO Uniform prototype
// // Example values
// let uniforms = Uniforms {
//     window_size: [800.0, 600.0],  // Your window size
//     center_pos: [400.0, 300.0],   // Center of your box
//     size: [200.0, 100.0],         // Width and height of your box
//     position: [400.0, 300.0],     // Position of your box
// };

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the color, coordinates and size.
#[derive(Debug,Clone,PartialEq,Default)]
pub struct RectSurface{
	pub position:Position,
	pub size:Size,
	pub color:Color,
}

impl RectSurface {
	pub fn new(x:f32,y:f32,width:f32,height:f32,color:Color) -> Self{
		let size = Size::new(width, height);
		let position = Position::new(x, y);
		Self { position,size,color }
	}

	pub fn color(&mut self,color:Color) {
		self.color = color
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let color = self.color.normalize();
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

impl Surface for RectSurface {
	fn draw(
		&self,
		render_pass:&mut wgpu::RenderPass,
		context: &RenderContext,
		state: &AppState
	) {
		let vertices = self.to_vertices();
		
		let vertex_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(&vertices), // TODO maybe remove bytemuck
			usage: wgpu::BufferUsages::VERTEX,
		});

		let center_pos = Position::new(
			self.position.x + self.size.width/2.0, 
			self.position.y + self.size.height/2.0
		);

		let bounds_buffer = state.device.create_buffer_init(
			&BufferInitDescriptor{
				label:Some("Bounds buffer"),
				contents: bytemuck::cast_slice(&[center_pos.x,center_pos.y]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}
		);

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
				label:Some("Rect bounds bind group"),
				layout:&context.rect_renderer.bounds_layout,
				entries:&[
					wgpu::BindGroupEntry{
						binding:0,
						resource:bounds_buffer.as_entire_binding()
					},
					wgpu::BindGroupEntry{
						binding:1,
						resource:size_buffer.as_entire_binding()
					},
					wgpu::BindGroupEntry{
						binding:2,
						resource:position_buffer.as_entire_binding()
					}
				]
			}
		);

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&context.rect_renderer.render_pipeline);
		render_pass.set_bind_group(0, &context.rect_renderer.window_bind_group, &[]);
		render_pass.set_bind_group(1, &bound_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

		render_pass.draw(0..vertices.len() as u32, 0..1);
	}

	fn position(&mut self, x:f32,y:f32){
		self.position = Position::new(x, y);
	} 
	
	fn get_position(&self) -> Position {
		self.position
	} 

	fn size(&mut self,width:f32,height:f32){
		self.size.width = width;
		self.size.height = height;
	} 

	fn width(&mut self, width:f32) {
		self.size.width = width
	}

	fn height(&mut self, height:f32) {
		self.size.height = height
	}

	fn get_size(&self) -> Size {
		self.size
	}

	fn get_bounds(&self) -> Bounds{
		let position = self.get_position();
		let size = self.get_size();
		Bounds{
			x:[position.x,size.width],
			y:[position.y,size.height],
		}
	}
}
