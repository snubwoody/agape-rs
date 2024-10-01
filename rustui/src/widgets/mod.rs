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
	layout::{Constraint, IntrinsicSize, Layout, WidgetSize}, 
	surface::{
		rect::RectSurface, Surface
	}, utils::{Position, Size}, 
};


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

/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{
	pub surface:Box<dyn Surface>,
	pub layout:Layout,
	pub children:Vec<Box<WidgetBody>>,
	pub constraint:Constraint,
	pub intrinsic_size:IntrinsicSize
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
		let size = self.layout.arrange_widgets(
			&mut self.children,
			Size::new(window.inner_size().width as f32, window.inner_size().height as f32),
			Position::new(
				self.surface.get_position().0, 
				self.surface.get_position().1
			)
		);

		// Set the size of the root widget
		match self.intrinsic_size.width {
			WidgetSize::Fill => {
				self.surface.width(window.inner_size().width as f32);
			},
			WidgetSize::Fit => {
				self.surface.width(size.width);
			},
			WidgetSize::Fixed(size) => {
				self.surface.width(size);
			}
		}

		match self.intrinsic_size.height {
			WidgetSize::Fill => {
				self.surface.height(window.inner_size().height as f32);
			},
			WidgetSize::Fit => {
				self.surface.height(size.height);
			},
			WidgetSize::Fixed(size) => {
				self.surface.height(size);
			}
		}
		
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
			constraint:Constraint::default(),
			intrinsic_size: Default::default()
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
