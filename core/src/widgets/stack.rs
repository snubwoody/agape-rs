use glium::{
	glutin::surface::WindowSurface, Display, Frame,  
};
use properties::Drawable;
use winit::window::Window;
use crate::colour::Colour;
use crate::{surface::Surface, app::view::RenderContext, widgets::Widget};
use crate::layout::{Layout};
use crate::widgets::WidgetBody;


#[derive(Debug)]
pub struct VStack{
	spacing:u32,
	padding:u32,
	children:Vec<Box<dyn Widget>>
}

#[derive(Drawable,Debug)]
pub struct HStack{
	surface:Surface,
	layout:Layout,
	children:Vec<Box<dyn Widget>>
}

impl HStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, Colour::Rgb(255, 255, 255));
		let layout = Layout::Horizontal { spacing:0, padding: 0 };
		Self { surface,layout,children }
	}
}


