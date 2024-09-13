use std::io::Cursor;
use text_to_png::{Size as ImageSize, TextRenderer};
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
	utils::{Bounds, Position,Size}, 
	vertex::Vertex
};

// TODO make a size struct
// FIXME change the colour to Colour enum
/// A rasterized texture of text  
#[derive(Debug,Clone)]
pub struct TextSurface{
	position:Position,
	size:Size,
	text:String,
	font_size:u8,
	colour:String,
	img:Vec<u8>
}

impl TextSurface {
	pub fn new(text:&str,colour:&str,font_size:u8) -> Self{
		let text_renderer = TextRenderer::default();

		let image_size:ImageSize;

		// Render the text as a png
		let text_image = text_renderer.render_text_to_png_data(
			text, 
			font_size, 
			colour
		).unwrap();

		image_size = text_image.size;

		// Get the raw pixel values for the image
		let img = image::load(
			Cursor::new(text_image.data), 
			image::ImageFormat::Png
		).unwrap().to_rgba8().into_raw();
		
		Self {
			position:Position::new(0.0, 0.0), 
			size:Size::new(image_size.width as f32, image_size.height as f32),
			text:String::from(text), 
			font_size, 
			colour:String::from(colour),
			img
		}
	}
	
	/// Rasterize the text and store the texture 
	pub fn build(&self,display:&Display<WindowSurface>) -> Texture2d{
		// Create an opengl raw image 
		let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
			&self.img,(self.size.width as u32,self.size.height as u32)
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

impl Surface for TextSurface {
	fn draw(
		&self,
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

		let vertices:Vec<Vertex> = self.to_vertices(self.size.width as i32, self.size.height as i32);
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

	fn size(&mut self,width:f32,height:f32) {
		self.size.width = width;
		self.size.height = height;
	}
	
	fn get_size(&self) -> Size {
		self.size
	}

	fn get_bounds(&self) -> Bounds {
		Bounds{
			x:[self.position.x,self.position.x + self.size.width],
			y:[self.position.y,self.position.y + self.size.height]
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
