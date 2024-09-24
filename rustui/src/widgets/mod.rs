pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
pub mod list;
pub mod image;
pub mod flex;
use std::{
	collections::HashMap, fmt::Debug,
};
use fontdue::layout::HorizontalAlign;
use glium::{
	glutin::{surface::WindowSurface}, Display, Frame 
};

use winit::window::Window;
use crate::{
	app::view::RenderContext, 
	layout::{IntrinsicSize, Layout}, 
	surface::{
		rect::RectSurface, Surface
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
	/// *Deprecated*.
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
		// Draw the widget to the screen
		self.surface.draw(display, frame, window, context);
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
			constraint:IntrinsicSize::Fit{padding:0}
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
	root_id:WidgetID
}

impl WidgetTree {
	pub fn new() -> Self{
		Self{
			nodes:vec![],
			root_id:0
		}
	}

	pub fn add(&mut self,node:Node){
		self.nodes.push(node);
	}

	pub fn root(&mut self,id:WidgetID){
		self.root_id = id
	}

	/// Lookup a [`Node`] by it's id return a reference to 
	/// the node.
	fn lookup(&self,id:WidgetID) -> Option<&Node>{
		for (_,node) in self.nodes.iter().enumerate(){
			if node.id == id {
				return Some(node)
			}
		}
		None
	}

	/// Lookup a [`Node`] by it's id and return a 
	/// mutable reference to the node.
	fn lookup_mut(&mut self,id:WidgetID) -> Option<&mut Node>{
		for (_,node) in self.nodes.iter_mut().enumerate(){
			if node.id == id {
				return Some(node)
			}
		}
		None
	}

	pub fn layout_pass(&mut self,window:&Window){
		// Store the position of all widgets to retrieve later
		let mut position_cache:HashMap<WidgetID, Position> = HashMap::new();

		for (_,node) in self.nodes.iter().enumerate(){
			match node.body.layout {
				Layout::Horizontal { spacing, padding } => {

					let mut total_size = Size::new((padding * 2) as f32, 0.0);
					// The positions to set the current widget
					let mut x_position = padding as f32;
					let y_position = node.body.surface.get_position().1 + padding as f32;

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
				Layout::Block { padding } => {
					let parent_size = node.body.surface.get_size();
					let parent_position = node.body.surface.get_position();
					for (_,edge) in node.edges.iter().enumerate(){
						match edge {
							Edge::Child(id) => {
								let child = self.lookup(*id).unwrap();

								// Find the difference between the parent and
								// the child size.
								let child_size = child.body.surface.get_size();
								let difference = parent_size - child_size;

								// Divide by two to put it in the center instead of 
								// the lower right corner
								let child_position = Position::new(
									(parent_position.0 + difference.width)/2.0,
									(parent_position.1 + difference.height)/2.0
								);

								position_cache.insert(*id, child_position);
							},
							_ => {}
						}
					}
				}
				_ => {}
			}
		}

		for (_,(id,position)) in position_cache.iter().enumerate(){
			match self.lookup_mut(*id){
				Some(node) => {
					node.body.surface.position(position.x, position.y);
				}
				None => {}
			}
		}
	}

	fn size_pass(&mut self,window:&Window){
		let mut max_sizes:HashMap<usize,Size> = HashMap::new();

		let mut max_size = Size::new(0.0, 0.0);
		max_size.width = window.inner_size().width as f32;
		max_size.height = window.inner_size().height as f32;
		max_sizes.insert(self.root_id, max_size);

		let child_size = self.get_constraints(self.root_id,&max_size);
		let root = self.lookup_mut(self.root_id).unwrap();
		match root.body.constraint{
			IntrinsicSize::Fill{width,height} => {
				if width && height {
					root.body.surface.size(max_size.width, max_size.height);
				}
				else if width {
					root.body.surface.size(max_size.width, child_size.height);
				}
				else if height {
					root.body.surface.size(child_size.width, max_size.height);
				}
			},
			IntrinsicSize::FillWidth => {
				root.body.surface.size(max_size.width, child_size.height);
			},
			IntrinsicSize::Fit{padding} => {
				root.body.surface.size(child_size.width + padding as f32, child_size.height + padding as f32);
			},
			IntrinsicSize::Fixed(width,height) => {
				root.body.surface.size(width, height);
			}
			_ => {}
		}
	}

	/// Size the children and return their size
	fn get_constraints(&self,id:WidgetID,max_size:&Size) -> Size {
		let node = self.lookup(id).unwrap();
		let mut max_height:f32 = 0.0;
		let mut max_width:f32 = 0.0;
		for (_,edge) in node.edges.iter().enumerate(){
			dbg!(edge);
			match edge {
				Edge::Child(id) => {
					let node = self.lookup(*id).unwrap();
					match node.body.constraint{
						IntrinsicSize::Fill{width,height} => {

						},
						IntrinsicSize::Fit{padding} => {

						},
						IntrinsicSize::Fixed(width,height) => {
							max_height = max_height.max(height);
							max_width = max_width.max(width);
						}
						_ => {}
					}
				},
				_ => {}
			}
		}

		Size::new(max_width, max_height)
	}

	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.size_pass(window);
		self.layout_pass(window);
		for (_,node) in self.nodes.iter().enumerate(){
			node.body.render(display, frame, window, context);
		}
	}
}
