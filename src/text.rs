use std::{fs::File, io::{Cursor, Read, Write}};
use glium::{
	glutin::surface::WindowSurface, index, Display, Program, Surface 
};
use text_to_png::TextRenderer;
use winit::window::Window;

pub fn render_text(display:&Display<WindowSurface>,program:&Program,window:&Window){
	
	let text_renderer = TextRenderer::default();
	let text_image = text_renderer.render_text_to_png_data("Hello world", 64, "#F24F31").unwrap();
	let image_size = text_image.size;
	let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png).unwrap().to_rgba8().into_raw();

	let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img,(image_size.width,image_size.height));
	let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
	
	let mut frame = display.draw();
	frame.clear_color(1.0, 1.0, 1.0, 1.0);
	
	let screen_size = window.inner_size();
	let uniforms = uniform! {
		width:screen_size.width as f32,
		height:screen_size.height as f32,
		tex: &texture,
	};

	let vertex_buffer = glium::VertexBuffer::new(
		display, 
		&[		
			Vertex::new(0,0,[0.0,1.0]), //Top left
			Vertex::new(500,0,[1.0,1.0]), // Top right
			Vertex::new(0, 500,[0.0,0.0]), //Bottom left
			Vertex::new(500,0,[1.0,1.0]), //Top right
			Vertex::new(0, 500,[0.0,0.0]), // Bottom left
			Vertex::new(500, 500,[1.0,0.0]), //Bottom right
	]).unwrap();

	let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);
			
	frame.draw(&vertex_buffer, &indices, program, &uniforms, &Default::default()).unwrap();
	frame.finish().unwrap();
}



#[derive(Debug,Clone,Copy)]
struct Vertex{
	position: [i32;2],
	uv:[f32;2]
}

impl Vertex {
	fn new(x:i32,y:i32,coords:[f32;2]) -> Self{

		Self { 
			position: [x,y],
			uv:coords
		}
	}
}

implement_vertex!(Vertex,position,uv);
