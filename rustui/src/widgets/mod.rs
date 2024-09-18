pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
pub mod list;
pub mod image;
pub mod flex;
use std::{collections::HashMap, fmt::Debug};
use glium::{
	glutin::surface::WindowSurface, texture::srgb_cubemap, Display, Frame 
};
use winit::window::Window;
use crate::{
	app::{
		events::EventFunction,
		view::RenderContext
	}, layout::{IntrinsicSize, Layout}, surface::{
		rect::RectSurface, Surface
	}, utils::Size
};


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
	pub constraint:IntrinsicSize
	//pub events:Vec<EventFunction>
}

impl WidgetBody {
	/// Draw the [`WidgetBody`] to the screen.
	pub fn render(
		&self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		// Render the parent then the children
		self.surface.draw(display, frame, window, context);
		self.children.iter().for_each(|child|child.render(display, frame, window, context));
	}

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
			constraint:IntrinsicSize::Fit
			//events:vec![]
		}
	}
}

#[derive(Debug)]
pub struct WidgetNode {
	pub body:WidgetBody,
	pub parent:Option<usize>,
	pub children:Vec<usize>
}

/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering, layouts and events are processed from.
#[derive(Debug)]
pub struct WidgetTree{
	root:usize,
	widgets:HashMap<usize,WidgetNode>
}

impl WidgetTree where  {
	pub fn new() -> Self {
		Self { 
			root:0,
			widgets:HashMap::new()
		}
	}

	pub fn build(mut self,widget:impl Widget + 'static) -> Self {
		/* let body = widget.build();
		for (index,child) in body.children.iter().enumerate(){
			let node = WidgetNode{
				parent:Some(0),
				body:child,
				children:vec![]
			};
			self.widgets.insert(index + 1, node);
		} */
		//self.widgets.insert(0, body);
		self
	}

	/* /// Returns an iterator for the [`WidgetTree`].
	pub fn iter(&self) -> WidgetTreeIter {
		WidgetTreeIter{
			stack:vec![&self.root_widget]
		}
	} */

	/// Draw the [`WidgetTree`] to the screen.
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		//self.widgets.get(&0)
	}
}

/// An [`Iterator`] for the [`WidgetTree`].
pub struct WidgetTreeIter<'a>{
	stack:Vec<&'a WidgetBody>
}

impl<'a> Iterator for WidgetTreeIter<'a> {
	type Item = &'a WidgetBody;

	fn next(&mut self) -> Option<Self::Item> {
		// Get the current widget from the top of the stack
		let widget = self.stack.pop();

		// Add the widgets children to the stack in reverse
		// for a depth first search
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



