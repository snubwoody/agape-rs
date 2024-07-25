use std::{fs::File, io::{Cursor, Write}};
use fontdue::{self, Font};
use glium::{
	glutin::surface::WindowSurface, 
	Display, 
	Program, 
};

pub fn render_text(display:&Display<WindowSurface>,program:&Program){
	
	// Read the font data.
	let font = include_bytes!("../fonts/Inter/static/Inter-Regular.ttf") as &[u8];
	// Parse it into the font type.
	let inter = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

	let (metrics, mut bitmap) = inter.rasterize('H', 48.0);

	let mut frame = display.draw();	

	let img = image::ImageReader::open("../rgb.png").unwrap().decode().unwrap();
	img.write_to(&mut Cursor::new(&mut bitmap), image::ImageFormat::Png).unwrap();
	
	dbg!(&bitmap);
	
	let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&bitmap, (metrics.width as u32, metrics.height as u32));
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
	
	let uniforms = uniform! { tex: &texture };
	let vertex_buffer = glium::VertexBuffer::new(
		display, 
		&[		// top-left
			Vertex::new(0,0,  [0.0,0.0]),
			Vertex::new(500,0,  [1.0,0.0]),
			Vertex::new(0,500,  [0.0,1.0]),
			Vertex::new(500,500,  [1.0,1.0]),
	]).unwrap();

	let index_buffer = glium::IndexBuffer::new(
		display, 
		glium::index::PrimitiveType::TrianglesList,
		&[
			0u16, 1, 2,
			1, 2, 3,
	]).unwrap();
			
	//frame.draw(&vertex_buffer, &index_buffer, program, &uniforms, &Default::default()).unwrap();
	//frame.finish().unwrap();
}



#[derive(Debug,Clone,Copy)]
struct Vertex{
	position: [i32;2],
	tex_coords:[f32;2]
}

impl Vertex {
	fn new(x:i32,y:i32,coords:[f32;2]) -> Self{

		Self { 
			position: [x,y],
			tex_coords:coords
		}
	}
}

implement_vertex!(Vertex,position,tex_coords);
