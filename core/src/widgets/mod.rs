pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::app::view::RenderContext;
use crate::layout::Layout;
use crate::utils::Bounds;
use crate::Surface;


/// Widget trait that all widgets must inherit from
pub trait Widget{
	fn build(self) -> WidgetBody;
}

pub struct WidgetBody{
	surface:Surface,
	layout:Layout,
	children:Vec<Box<WidgetBody>>,
	events:Vec<Box<dyn Fn()>>
}

impl WidgetBody {
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		// Arrange the children
		self.arrange_widgets();

		// Render the parent and the child
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|widget|widget.render(display, frame, window, context));
	}

	pub fn on_hover(&mut self) {
		for (_,event) in self.events.iter().enumerate(){
			event()
		}
	}

	pub fn arrange_widgets(&mut self) {
		// Arrange the children
		let position = self.get_position();
		self.children.iter_mut().for_each(|widget| {
			widget.arrange_widgets();}
		);
		let size = self.layout.arrange([position.0,position.1],&mut self.children);
		self.size(size.0, size.1);
	}

	/// Set the position of the [`Widget`]
	pub fn position(&mut self, x:i32,y:i32){
		self.surface.x = x;
		self.surface.y = y;
	} 
	
	/// Get the [`Widget`] position
	pub fn get_position(&self) -> (i32,i32){
		(self.surface.x,self.surface.y)
	} 

	/// Set the size of the [`Widget`]
	pub fn size(&mut self,width:u32,height:u32){
		self.surface.width = width;
		self.surface.height = height;
	} 

	/// Get the size of the [`Widget`]
	pub fn get_size(&self) -> (u32,u32){
		(self.surface.width,self.surface.height)
	}

	pub fn get_bounds(&self) -> Bounds{
		let position = self.get_position();
		let size = self.get_size();
		Bounds{
			x:[position.0,size.0 as i32],
			y:[position.1,size.1 as i32],
		}
	}

}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Surface::default();
		let layout = Layout::Single { padding: 0 };
		Self { surface, layout, children: vec![], events: vec![] }
	}
}


// TODO maybe implement iter for the widget tree
pub struct WidgetTree{
	pub widgets:Vec<WidgetBody>,
}

impl WidgetTree where  {
	pub fn new() -> Self{
		Self { widgets: Vec::new() }
	}

	pub fn add(&mut self,widget:impl Widget + 'static) {
		let node = widget.build();

		self.widgets.push(node);
	}

	/// Build the widget tree 
	pub fn build(&mut self,widget:impl Widget + 'static) {
		self.add(widget);
	}

	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.widgets.iter_mut().for_each(|widget| {
			widget.render(display, frame, window, context)
		})
	}
}

