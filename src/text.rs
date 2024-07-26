use std::{io::Cursor, str::Chars};
use glium::{
	glutin::surface::WindowSurface, index, texture::RawImage2d, Blend, Display, DrawParameters, Program, Surface, Texture2d, VertexBuffer 
};
use text_to_png::{Size, TextRenderer};
use winit::window::Window;
use crate::{colour::rgb, Vertex};

pub fn render_text(display:&Display<WindowSurface>,program:&Program,window:&Window){
	let mut frame = display.draw();
	frame.clear_color(1.0,1.0,1.0,1.0);
	
	let text = TextSurface::new(0, 0, "Hello world", 64,16,display);
	text.render(display, &mut frame, window, program);
	
	frame.finish().unwrap();
}

//TODO change all size, position and colours from i32 to u32 
// TODO maybe change this to a build step instead of when creating
// FIXME whitespace cannot be rendered
/// An array of [`CharSurface`] rendered as a word
pub struct TextSurface{
	x:i32,
	y:i32,
	text:Vec<CharSurface>,
	font_size:u8,
	letter_spacing:i32,
}

impl TextSurface {
	pub fn new(x:i32,y:i32,text:&str,font_size:u8,spacing:i32,display:&Display<WindowSurface>) -> Self {
		let mut letters:Vec<CharSurface> = vec![];
		let mut x_offset = 0;
		text.chars().for_each(|character|{
			let mut char_surface = CharSurface::new(x+x_offset,y, character, font_size);
			char_surface.build(display);
			let size = char_surface.size.expect("Null size");
			x_offset += size.width as i32 + spacing;
			letters.push(char_surface);
		});

		Self {
			x,
			y,
			text:letters,
			font_size,
			letter_spacing:spacing
		}
	}

	pub fn render(
		&self,
		display:&Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&glium::Program,
	) {
		//FIXME whitespace returns error
		self.text.iter().for_each(|letter|letter.render(display, frame, window, program))		
	}
}

/// A single character rendered onto a surface.  
/// After making new character call the [`build`] method
/// to rasterize it and store the texture for use when rendering
#[derive(Debug)]
pub struct CharSurface{
	pub x:i32,
	pub y:i32,
	pub character:char,
	pub font_size:u8,
	size:Option<Size>,
	texture:Option<Texture2d>
}

impl CharSurface {
	pub fn new(x:i32,y:i32,character:char,font_size:u8) -> Self{
		Self { x, y, character, font_size, size:None, texture:None }
	}

	pub fn build(&mut self,display:&Display<WindowSurface>) -> &Self{
		let (texture,size) = self.rasterize(display);
		self.texture = Some(texture);
		self.size = Some(size);
		self
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

		let texture = self.texture.as_ref().expect("Null texture, call build before render");
		let size = self.size.as_ref().expect("Null size, call build before render");

		let uniforms = uniform! {
			width:screen_width,
			height:screen_height,
			tex: texture,
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
		let raw_image:RawImage2d<_>;
		let image_size:Size;
		match self.character.to_string().as_str() {
			" " => {
				let mut img = image::RgbaImage::new(self.font_size as u32, self.font_size as u32);
				for x in 0..self.font_size{
					for y in 0..self.font_size{
						img.put_pixel(x as u32, y as u32, image::Rgba([0,0,0,0]))
					}
				}
				image_size = Size::new(self.font_size as u32, self.font_size as u32);
				raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(),(self.font_size as u32,self.font_size as u32));
			}
			letter => {
				let text_image = text_renderer.render_text_to_png_data(self.character.to_string(), self.font_size, "#F24F31").unwrap();
				image_size = text_image.size;
				let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png).unwrap().to_rgba8().into_raw();
				raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img,(image_size.width,image_size.height));

			}
		}
		
		
	
		let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();

		return (texture,image_size);
	}
}
