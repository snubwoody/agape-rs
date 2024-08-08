pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
use std::fmt::Debug;
use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::app::view::RenderContext;
use crate::layout::Layout;
use crate::Surface;


/// Represents anything that's drawable to the screen ie.
/// it must have a size and a position
pub trait Drawable{
	/// Set the position of the [`Widget`]  
	/// Note that positions start from the upper left 
	/// corner
	fn position(&mut self, x:i32,y:i32); 
	
	/// Get the [`Widget`] position
	fn get_position(&self) -> (i32,i32); 

	/// Set the size of the [`Widget`]
	fn size(&mut self,width:u32,height:u32); 

	/// Get the size of the [`Widget`]
	fn get_size(&self) -> (u32,u32);
}

/// Widget trait that all widgets must inherit from
pub trait Widget:Debug{
	fn build(&self) -> WidgetBody;

	fn position(&mut self, x:i32,y:i32){} 
	
	/// Get the [`Widget`] position
	fn get_position(&self) -> (i32,i32){(0,0)} 

	/// Set the size of the widget
	fn size(&mut self,width:u32,height:u32){} 

	/// Get the size of the widget
	fn get_size(&self) -> (u32,u32){(0,0)}
}

#[derive(Debug)]
pub struct WidgetBody{
	surface:Surface,
	layout:Layout,
	children:Vec<Box<WidgetBody>>
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

	pub fn arrange_widgets(&mut self) {
		// Arrange the children
		let position = self.get_position();
		self.children.iter_mut().for_each(|widget| {
			dbg!("Hello",&widget);
			let child_position = widget.get_position();
			widget.arrange_widgets();}
		);
		let size = self.layout.arrange([position.0,position.1],&mut self.children);
		self.size(size.0, size.1);
		dbg!("Arranged widget",&self);
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

}

type WidgetID = usize;

#[derive(Debug)]
pub struct WidgetTree{
	widgets:Vec<WidgetBody>,
	root:WidgetID,
	next:WidgetID
}

impl WidgetTree where  {
	pub fn new() -> Self{
		Self { widgets: Vec::new(), root: 0, next: 0 }
	}

	pub fn add(&mut self,widget:impl Widget + 'static) {
		let parent:Option<WidgetID>;

		if self.next == 0 {
			parent = None;
		}
		else {
			parent = Some(self.next)
		}

		let node = widget.build();

		self.widgets.push(node);
		self.next += 1;
	}

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
		/* let widgets = vec![];
		widgets.push(self)

		self.widgets.iter_mut().rev().for_each(|widget|widget.arrange()); */
		self.widgets.iter_mut().for_each(|widget| {
			widget.render(display, frame, window, context)
		})
	}
}

