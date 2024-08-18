use std::io::Cursor;
use image::{imageops::FilterType, GenericImageView};
use text_to_png::{Size, TextRenderer};
use glium::{
	glutin::surface::WindowSurface, 
	index, 
	Blend, 
	Display, 
	DrawParameters, 
	Surface as GliumSurface, 
	Texture2d, 
	VertexBuffer 
};
use crate::{
	app::view::RenderContext, 
	colour::rgb, 
	surface::Surface, 
	utils::{Bounds, Position}, 
	vertex::Vertex
};


///TODO
#[derive(Debug)]
pub struct ImageSurface{
	position:Position,
	width:u32,
	height:u32,
	img:Vec<u8>
}

impl ImageSurface {
	pub fn new(path:&str,width:u32,height:u32) -> Self{

		// Get the raw pixel values for the image
		let img = image::ImageReader::open(path).unwrap().decode().unwrap();
		
		let raw_image = img.resize(width, height, FilterType::Gaussian).to_rgba8().into_raw();
		
		Self {
			position:Position::new(0.0, 0.0), 
			width,
			height,
			img:raw_image
		}
	}
	
	/// Rasterize the text and store the texture 
	pub fn build(&mut self,display:&Display<WindowSurface>) -> Texture2d{
		// Create an opengl raw image 
		let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
			&self.img,(self.width,self.height)
		);

		// Create the texture from the image
		let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();

		return texture;

	}

	fn to_vertices(&self,width:i32,height:i32) -> Vec<Vertex>{
		let colour = rgb(255, 255, 255);
		let x = self.position.x as i32;
		let y = self.position.y as i32;

		let vertex1 = Vertex::new_with_texture(x,y,colour,[0.0,1.0]); //Top left
		let vertex2 = Vertex::new_with_texture(x+width,y,colour,[1.0,1.0]); // Top right
		let vertex3 = Vertex::new_with_texture(x, y+height,colour,[0.0,0.0]); //Bottom left
		let vertex4 = Vertex::new_with_texture(x+width,y,colour,[1.0,1.0]); //Top right
		let vertex5 = Vertex::new_with_texture(x, y+height,colour,[0.0,0.0]); // Bottom left
		let vertex6 = Vertex::new_with_texture(x+width, y+height,colour,[1.0,0.0]); //Bottom right
	
		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}
}

impl Surface for ImageSurface {
	fn draw(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext,
	) {
		let params = DrawParameters{
			blend:Blend::alpha_blending(),
			..Default::default()
		};

		let screen_width = window.inner_size().width as f32;
		let screen_height = window.inner_size().height as f32;

		let texture = self.build(display);

		let uniforms = uniform! {
			width:screen_width,
			height:screen_height,
			tex: texture,
		};

		let vertices:Vec<Vertex> = self.to_vertices(self.width as i32, self.height as i32);
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		
		frame.draw(
			&vertex_buffer, 
			&indices, 
			&context.text_program, 
			&uniforms,
			&params
		).unwrap();
		
	}

	fn size(&mut self,width:u32,height:u32) {
		self.width = width;
		self.height = height;
	}
	
	fn get_size(&self) -> (u32,u32) {
		(self.width,self.height)
	}

	fn get_bounds(&self) -> Bounds {
		Bounds{
			x:[self.position.x,self.position.x + self.width as f32],
			y:[self.position.y,self.position.y + self.height as f32]
		}
	}

	fn position(&mut self, x:f32,y:f32) {
		self.position.x = x;
		self.position.y = y;
	}

	fn get_position(&self) -> (f32,f32) {
		(self.position.x,self.position.y)
	}
}
