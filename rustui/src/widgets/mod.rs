pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
pub mod list;
pub mod image;
pub mod flex;
use std::fmt::Debug;
use glium::{
	glutin::surface::WindowSurface, texture::srgb_cubemap, Display, Frame 
};
use winit::window::Window;
use crate::{
	app::{
		events::EventFunction,
		view::RenderContext
	}, layout::{Layout, SizeConstraint}, surface::{
		rect::RectSurface, Surface
	}
};


// TODO change size to floating point values
// TODO change render name to draw

/// Widget trait that all widgets must inherit from.
pub trait Widget:Debug{
	/// Build the [`Widget`] into a primitive [`WidgetBody`]
	fn build(&self) -> WidgetBody;

	/// Get the children and consume the [`Widget`], since this is 
	/// the last step before the widget is turned to a 
	/// [`WidgetBody`].
	fn get_children(self) -> Vec<Box<dyn Widget>>;
}

/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{
	pub surface:Box<dyn Surface>,
	pub layout:Layout,
	pub children:Vec<Box<WidgetBody>>,
	pub constraint:SizeConstraint
	//pub events:Vec<EventFunction>
}

impl WidgetBody {
	/// Draw the [`WidgetBody`] to the screen.
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		// Arrange the children
		self.arrange_widgets();

		// Render the parent then the children
		self.surface.draw(display, frame, window, context);
		//self.children.iter_mut().for_each(|widget|widget.render(display, frame, window, context));
	}

	/// TODO
	pub fn arrange_widgets(&mut self) {
		// Arrange the children
		let position = self.surface.get_position();
		self.children.iter_mut().for_each(|widget| {
			widget.arrange_widgets();
		});

		let size = self.layout.arrange([position.0,position.1],&mut self.children);
		self.surface.size(size.0 as f32, size.1 as f32);
	}


}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());
		let layout = Layout::Single{ padding: 0 };

		Self { 
			surface, 
			layout, 
			children:vec![], 
			constraint:SizeConstraint::Fit
			//events:vec![]
		}
	}
}

// TODO maybe implement iter for the widget tree
/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering, layouts and events are processed from.
#[derive(Debug)]
pub struct WidgetTree{
	pub root_widget:WidgetBody,
}

impl WidgetTree where  {
	pub fn new(widget:impl Widget + 'static) -> Self{
		Self { 
			root_widget:widget.build(),
		}
	}

	/// Walk the [`WidgetTree`] by depth first.
	fn walk(
		&self,
		root_widget:&mut WidgetBody,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		root_widget.render(display, frame, window, context);
		for (_,child) in root_widget.children.iter_mut().enumerate(){
			dbg!(&child.layout);
			self.walk(&mut *child,display,frame,window,context);
		}
	}

	/// Returns an iterator for the [`WidgetTree`].
	pub fn iter(&self) -> WidgetTreeIter {
		WidgetTreeIter{
			stack:vec![&self.root_widget]
		}
	}

	/// Draw the [`WidgetTree`] to the screen.
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		//self.walk(&mut self.root_widget,display,frame,window,context);
		for (_,node) in self.iter().enumerate(){
			dbg!(&node.layout);
		}
	}
}

/// An [`Iterator`] for the [`WidgetTree`].
pub struct WidgetTreeIter<'a>{
	stack:Vec<&'a WidgetBody>
}

impl<'a> Iterator for WidgetTreeIter<'a> {
	type Item = &'a WidgetBody;

	fn next(&mut self) -> Option<Self::Item> {
		// Get the curr
		let widget = self.stack.pop();
		match widget {
			Some(current_widget) => {
				current_widget.children.iter().rev().for_each(|node|{
					self.stack.push(node)
				});
			},
			None => {}
		}
		widget
	}
}

