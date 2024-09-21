pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
pub mod list;
pub mod image;
pub mod flex;
use std::{
	collections::HashMap, fmt::Debug, hash::Hash
};
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
	utils::{Position, Size}
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

type WidgetID = usize;

#[derive(Debug)]
pub enum Edge{
	Parent(WidgetID),
	Child(WidgetID),
	Sibling(WidgetID)
}

#[derive(Debug)]
pub struct Node{
	pub id:usize,
	pub body:WidgetBody,
	pub edges:Vec<Edge>,
}

/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering, layouts and events are processed from.
#[derive(Debug)]
pub struct WidgetTree{
	nodes:Vec<Node>,
}

impl WidgetTree {
	pub fn new() -> Self{
		Self{
			nodes:vec![]
		}
	}

	pub fn add(&mut self,node:Node){
		self.nodes.push(node);
	}


	pub fn arrange(&mut self){
		let mut position_cache:HashMap<WidgetID, Position> = HashMap::new();

		for (_,node) in self.nodes.iter().enumerate(){
			match node.body.layout {
				Layout::Horizontal { spacing, padding } => {
					dbg!("Arranging layout");
					let mut total_size = Size::new((padding * 2) as f32, 0.0);
					let mut x_position = padding as f32;
					let y_position = node.body.surface.get_position().1;
					for (_,edge) in node.edges.iter().enumerate(){
						match edge {
							Edge::Child(id) => {
								let child = self.lookup(*id).unwrap();
								let size = child.body.surface.get_size();

								total_size += size;
								total_size.width += spacing as f32;
								
								position_cache.insert(*id, Position::new(x_position, y_position));
								x_position += spacing as f32 + size.width
							}
							_ => {}
						}
					}
				},
				_ => {}
			}
		}
		self.change_sizes(position_cache);
	}

	fn change_sizes(&mut self,position_cache:HashMap<WidgetID,Position>){
		for (_,(id,position)) in position_cache.iter().enumerate(){
			match self.lookup_mut(*id){
				Some(node) => {
					node.body.surface.position(position.x, position.y);
				}
				None => {}
			}
		}
	}

	/// Lookup a [`Node`] by it's id
	fn lookup(&self,id:WidgetID) -> Option<&Node>{
		for (_,node) in self.nodes.iter().enumerate(){
			if node.id == id {
				return Some(node)
			}
		}
		None
	}

	/// Lookup a [`Node`] by it's id and return a mutable reference
	fn lookup_mut(&mut self,id:WidgetID) -> Option<&mut Node>{
		for (_,node) in self.nodes.iter_mut().enumerate(){
			if node.id == id {
				return Some(node)
			}
		}
		None
	}

	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.arrange();
		for (_,node) in self.nodes.iter().enumerate(){
			node.body.render(display, frame, window, context);
		}
	}
}
