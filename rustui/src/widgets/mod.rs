pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
pub mod image;
pub mod flex;
use std::{
	collections::HashMap, fmt::Debug,
};
use glium::{
	glutin::surface::WindowSurface, 
	Display, 
	Frame 
};
use winit::window::Window;
use crate::{
	app::view::RenderContext, 
	layout::{IntrinsicSize, Layout, WidgetSize}, 
	surface::{
		rect::RectSurface, Surface
	}, 
	utils::{Position, Size}
};

type WidgetID = usize;

/// The trait that all widgets must implement.
pub trait Widget:Debug{
	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> WidgetBody;

	/// Get the children and consume the [`Widget`], since this is 
	/// the last step before the widget is turned to a 
	/// [`WidgetBody`].  
	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>>;
}

/// A node in the widget tree
#[derive(Debug)]
pub struct Node{
	pub id:usize,
	pub body:WidgetBody,
	pub parent:Option<WidgetID>,
	pub children: Vec<WidgetID>
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
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		// Calculate the sizes of the widget's children
		let width = self.constraint.width.size_widgets(window.inner_size().width as f32,&mut self.children);
		let height = self.constraint.height.size_widgets(window.inner_size().height as f32,&mut self.children);

		self.surface.width(width);
		self.surface.height(height);

		// Arrange the children
		self.layout.arrange_widgets(&mut self.children);
		
		// Draw the parent then the children to the screen
		self.surface.draw(display, frame, window, context);
		self.children.iter_mut().for_each(|child|{
			child.render(display, frame, window, context);
		});
	}
}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());
		let layout = Layout::Block{ padding: 0 };

		Self { 
			surface, 
			layout, 
			children:vec![], 
			constraint:IntrinsicSize { 
				width: WidgetSize::Fit(0.0), 
				height: WidgetSize::Fit(0.0) 
			}
			//events:vec![]
		}
	}
}


/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering and layouts processed from.
#[derive(Debug)]
pub struct WidgetTree{
	root:WidgetBody
}

impl WidgetTree {
	pub fn new(root:WidgetBody) -> Self{
		Self{root}
	}

	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.root.render(display, frame, window, context);
	}
}
