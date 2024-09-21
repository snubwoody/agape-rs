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
	glutin::surface::WindowSurface, Display, Frame 
};
use winit::window::Window;
use crate::{
	app::view::RenderContext, 
	colour::Colour,
	layout::{IntrinsicSize, Layout}, 
	surface::{
		image::ImageSurface, rect::RectSurface, text::TextSurface, Surface
	}, 
	utils::Size
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
		//self.children.iter().for_each(|child|child.render(display, frame, window, context));
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

pub enum Edge{
	Parent,
	Child,
	Sibling
}

pub struct Node{
	id:usize,
	body:WidgetBody,
	edges:Vec<Edge>,
}

#[derive(Debug)]
pub struct WidgetNode{
	pub id:usize,
	pub body:WidgetBody,
	pub parent:Option<usize>,
	pub children:Vec<usize>
}

/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering, layouts and events are processed from.
pub struct WidgetTree{
	root:usize,
	pub widgets:HashMap<usize,WidgetNode>,
}

impl WidgetTree {
	pub fn new() -> Self{
		WidgetTree { 
			root: 0, 
			widgets: HashMap::new(), 
		}
	}

	pub fn add(
		&mut self,
		widget:impl Widget + 'static,
		parent:Option<usize>,
		children:Vec<usize>,
		id:usize
	) {
		let _widget = WidgetNode{
			id,
			parent,
			children,
			body:widget.build(),
		};
		self.widgets.insert(id, _widget);
	}

	pub fn arrange(&self){
		
	}

	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.arrange();
		for (_,(id,widget)) in self.widgets.iter().enumerate(){
			widget.body.render(display, frame, window, context);
		}
	}
}
