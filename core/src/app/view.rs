use std::collections::HashMap;

use glium::{
	glutin::surface::WindowSurface,Display,Surface,Program
};
use winit::window::Window;
use crate::widgets::Widget;

/// A page-like structure that holds multiple widgets below it and renders them.  
/// It can only have one [`Widget`] child
#[derive(Debug)]
pub struct View<W:Widget>{
	pub child:W
}

impl<W> View<W> where W:Widget {
	pub fn new(child:W) -> Self{
		Self{child}
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

		//Render the children, passing the objects down the widget tree
		self.child.render(display,&mut frame,window,context);

		//Swap the buffers
		frame.finish().unwrap();
	}
}

/// Contains the compiled shader programs
#[derive(Debug)]
pub struct RenderContext{
	pub surface_program:Program,
	pub text_program:Program,
}

impl RenderContext {
	pub fn new(
		surface_program:Program,
		text_program:Program
	) -> Self {
		Self { surface_program, text_program }
	}
}


type WidgetID = usize;

#[derive(Debug)]
pub struct WidgetTree{
	widgets:HashMap<WidgetID,WidgetNode>,
	root:WidgetID,
	next:WidgetID
}

impl WidgetTree where  {
	pub fn new() -> Self{
		Self { widgets: HashMap::new(), root: 0, next: 0 }
	}

	pub fn add(mut self,widget:impl Widget + 'static) -> Self{
		let parent:Option<WidgetID>;
		if self.next == 0 {
			parent = None;
		}
		else {
			parent = Some(self.next)
		}

		let node = WidgetNode{
			widget:Box::new(widget),
			parent,
			children:vec![1]
		};

		self.widgets.insert(self.next, node);
		self.next += 1;
		self
	}
}

#[derive(Debug)]
pub struct WidgetNode{
	widget:Box<dyn Widget>,
	parent:Option<WidgetID>,
	children:Vec<WidgetID>
}


