use wgpu::util::DeviceExt;
use crate::{
	app::RenderContext, 
	colour::Colour, 
	surface::Surface, 
	utils::{Bounds, Position, Size}, 
	vertex::Vertex
};

// TODO change x and y to position
/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the colour, coordinates and size.
#[derive(Debug,Clone,PartialEq,Default)]
pub struct RectSurface{
	pub position:Position,
	pub size:Size,
	pub colour:Colour,
}

impl RectSurface {
	pub fn new(x:f32,y:f32,width:f32,height:f32,colour:Colour) -> Self{
		let size = Size::new(width, height);
		let position = Position::new(x, y);
		Self { position,size,colour }
	}

	pub fn colour(&mut self,colour:Colour) {
		self.colour = colour
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let colour = self.colour.normalize();
		let x = self.position.x;
		let y = self.position.y;

		let vertex1 = Vertex::new(x, y,colour); //Top left
		let vertex2 = Vertex::new(x+self.size.width, y,colour); // Top right
		let vertex3 = Vertex::new(x, y+self.size.height,colour); //Bottom left
		let vertex4 = Vertex::new(x+self.size.width, y,colour); //Top right
		let vertex5 = Vertex::new(x, y+self.size.height,colour); // Bottom left
		let vertex6 = Vertex::new(x+self.size.width, y+self.size.height,colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

}

impl Surface for RectSurface {
	fn draw(
		&self,
		render_pass:&mut wgpu::RenderPass,
		context: &RenderContext,
		device:&wgpu::Device
	) {
		let vertices = self.to_vertices();
		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(&vertices), // TODO maybe remove bytemuck
			usage: wgpu::BufferUsages::VERTEX,
		});

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&context.rect_pipeline);
		render_pass.set_bind_group(0, &context.window_bind_group, &[]);
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
