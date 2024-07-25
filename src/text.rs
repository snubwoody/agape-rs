use std::io::Cursor;
use glium::{
	glutin::surface::WindowSurface, 
	index, 
	Blend, 
	Display, 
	DrawParameters, 
	Program, 
	Surface, 
	VertexBuffer,
	Texture2d 
};
use text_to_png::{Size, TextRenderer};
use winit::window::Window;
use crate::{colour::rgb, Vertex};

pub fn render_text(display:&Display<WindowSurface>,program:&Program,window:&Window){
	
	let text_renderer = TextRenderer::default();
	let text_image = text_renderer.render_text_to_png_data("Hello world", 500, "#F24F31").unwrap();
	let image_size = text_image.size;
	let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png).unwrap().to_rgba8().into_raw();

	let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img,(image_size.width,image_size.height));
	let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
	
	let mut frame = display.draw();
	frame.clear_color(1.0, 1.0, 1.0, 1.0);
	/* 
	let screen_size = window.inner_size();

	let uniforms = uniform! {
		width:screen_size.width as f32,
		height:screen_size.height as f32,
		tex: &texture,
	};

	let vertex_buffer = glium::VertexBuffer::new(
		display, 
		&[		
			Vertex::new_with_texture(0,0,rgb(255, 255, 255),[0.0,1.0]), //Top left
			Vertex::new_with_texture(500,0,rgb(255, 255, 255),[1.0,1.0]), // Top right
			Vertex::new_with_texture(0, 500,rgb(255, 255, 255),[0.0,0.0]), //Bottom left
			Vertex::new_with_texture(500,0,rgb(255, 255, 255),[1.0,1.0]), //Top right
			Vertex::new_with_texture(0, 500,rgb(255, 255, 255),[0.0,0.0]), // Bottom left
			Vertex::new_with_texture(500, 500,rgb(255, 255, 255),[1.0,0.0]), //Bottom right
	]).unwrap();

	let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		
	frame.draw(&vertex_buffer, &indices, program, &uniforms, &Default::default()).unwrap(); */

	let c = CharSurface::new(0, 0, 'c', 16);
	c.render(display, &mut frame, window, program);
	frame.finish().unwrap();
}

/// A single character rendered onto a surface
#[derive(Debug,Clone)]
pub struct CharSurface{
	pub x:i32,
	pub y:i32,
	pub character:char,
	pub font_size:i8,
}

impl CharSurface {
	pub fn new(x:i32,y:i32,character:char,font_size:i8) -> Self{
		Self { x, y, character, font_size }
	}

	pub fn render(
		&self,
		display:&Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&glium::Program,
	) {
		
		//FIXME blending not working properly
		let params = DrawParameters{
			blend:Blend::alpha_blending(),
			..Default::default()
		};

		let screen_width = window.inner_size().width as f32;
		let screen_height = window.inner_size().height as f32;

		let (texture,size) = self.rasterize(display);

		let uniforms = uniform! {
			width:screen_width,
			height:screen_height,
			tex: &texture,
		};

		let vertices:Vec<Vertex> = self.to_vertices(size.width as i32, size.height as i32);
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		
		frame.draw(
			&vertex_buffer, 
			&indices, 
			&program, 
			&uniforms,
			&params
		).unwrap();


	}

	fn to_vertices(&self,width:i32,height:i32) -> Vec<Vertex>{
		let colour = rgb(255, 255, 255);

		let vertex1 = Vertex::new_with_texture(self.x,self.y,colour,[0.0,1.0]); //Top left
		let vertex2 = Vertex::new_with_texture(self.x+width,self.y,colour,[1.0,1.0]); // Top right
		let vertex3 = Vertex::new_with_texture(self.x, self.y+height,colour,[0.0,0.0]); //Bottom left
		let vertex4 = Vertex::new_with_texture(self.x+width,self.y,colour,[1.0,1.0]); //Top right
		let vertex5 = Vertex::new_with_texture(self.x, self.y+height,colour,[0.0,0.0]); // Bottom left
		let vertex6 = Vertex::new_with_texture(self.x+width, self.y+height,colour,[1.0,0.0]); //Bottom right
	
		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

	fn rasterize(&self,display:&Display<WindowSurface>) -> (Texture2d,Size) {
	
		let text_renderer = TextRenderer::default();
		let text_image = text_renderer.render_text_to_png_data(self.character.to_string(), 64, "#F24F31").unwrap();
		let image_size = text_image.size;
		let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png).unwrap().to_rgba8().into_raw();
	
		let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img,(image_size.width,image_size.height));
		let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();

		return (texture,image_size);
	}
}
