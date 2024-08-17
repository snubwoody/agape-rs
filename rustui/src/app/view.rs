use glium::{
	glutin::surface::WindowSurface,
	Display,
	Surface,
	Program,
};
use winit::window::Window;
use crate::widgets::{Widget, WidgetTree};

/// A page
pub struct View{
	pub widget_tree:WidgetTree
}

impl View {
	pub fn new(root_widget:impl Widget + 'static) -> Self {
		let mut widget_tree = WidgetTree::new();
		widget_tree.build(root_widget);
		
		Self { widget_tree }
	}
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		window:&Window,
		context:&RenderContext
	) {
		// Create a frame that will be drawn to
		let mut frame = display.draw();
		frame.clear_color(1.0, 1.0, 1.0, 1.0);

		//Render the widget tree
		self.widget_tree.render(display,&mut frame,window,context);

		//Swap the buffers
		frame.finish().unwrap();
	}
}

/// Contains the compiled shader programs
#[derive(Debug)]
pub struct RenderContext{
	pub surface_program:Program,
	pub text_program:Program,
	pub image_program:Program
}

impl RenderContext {
	pub fn new(
		surface_program:Program,
		text_program:Program,
		image_program:Program
	) -> Self {
		Self{ 
			surface_program, 
			text_program,
			image_program
		}
	}
}






