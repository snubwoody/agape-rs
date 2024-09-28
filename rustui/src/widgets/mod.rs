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
	layout::{IntrinsicSize, Layout, LayoutManager}, 
	surface::{
		rect::RectSurface, Surface
	}, 
	utils::{Position, Size}
};

type WidgetID = usize;

/// Widget trait that all widgets must inherit from.
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

impl Node {
	fn size(&mut self,widget_tree:&mut WidgetTree,max_size:Size) {
		
		for (_,edge) in self.children.iter().enumerate(){
			let child = widget_tree.lookup_mut(*edge).unwrap();
		}
		match self.body.constraint {
			IntrinsicSize::Fill { width, height } => {
				self.body.surface.size(
					max_size.width, 
					max_size.height
				);
			},
			IntrinsicSize::Fit { padding } => {

			}
			_ => {}
		}
	}
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

}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());
		let layout = Layout::Block{ padding: 0 };

		Self { 
			surface, 
			layout, 
			children:vec![], 
			constraint:IntrinsicSize::Fit{padding:0}
			//events:vec![]
		}
	}
}


/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering and layouts processed from.
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

	pub fn build(&mut self,widget:impl Widget + 'static) {
		let root = widget.build();
		let children = self.add_edges(Box::new(widget),0);
		self.nodes.push(Node{
			id:0,
			body:root,
			parent:None,
			children:children,
		});
		
	}
	
	pub fn add_edges(&mut self,parent:Box<dyn Widget>,id:WidgetID) -> Vec<WidgetID> {
		let children = parent.get_children();
		let mut edges = vec![];
		
		for (_,child) in children.into_iter().enumerate(){
			let child_id = rand::random::<usize>();
			edges.push(child_id);

			let child = Node { 
				id: child_id, 
				body: child.build(),
				parent:Some(id),
				children:self.add_edges(child, child_id)
			};

			self.nodes.push(child);
			
		}
		edges
	}

	/// Look up a [`Node`] by it's id return a reference to 
	/// the node.
	fn lookup(&self,id:WidgetID) -> Option<&Node>{
		for (_,node) in self.nodes.iter().enumerate(){
			if node.id == id {
				return Some(node)
			}
		}
		None
	}

	/// Look up a [`Node`] by it's id and return a 
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

					for (_,edge) in node.children.iter().enumerate(){
						let child = self.lookup(*edge).unwrap();
						let size = child.body.surface.get_size();
						
						total_size += size;
						total_size.width += spacing as f32;
						
						position_cache.insert(*edge, Position::new(x_position, y_position));
						x_position += spacing as f32 + size.width
					}
				},
				Layout::Block { padding } => {
					let parent_size = node.body.surface.get_size();
					let parent_position = node.body.surface.get_position();
					for (_,edge) in node.children.iter().enumerate(){
							let child = self.lookup(*edge).unwrap();
							
							// Find the difference between the parent and
							// the child size.
							let child_size = child.body.surface.get_size();
							let difference = parent_size - child_size;
							let child_position = Position::new(
								// Divide by 2 to put it in the center
								(parent_position.0 + difference.width)/2.0, 
								(parent_position.1 + difference.height)/2.0
							);
							position_cache.insert(*edge, child_position);
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

		let window_size = Size::new(
			window.inner_size().width as f32, 
			window.inner_size().height as f32
		);

		// Set the max size of the root widget to the 
		// size of the window.
		max_sizes.insert(self.root_id, window_size);

		// Get the max size that the children will occupy
		let child_size = self.get_constraints(self.root_id,&window_size);
		let root = self.lookup_mut(self.root_id).unwrap();

		match root.body.constraint{
			IntrinsicSize::Fill{width,height} => {
				if width && height {
					root.body.surface.size(
						window.inner_size().width as f32, 
						window.inner_size().height as f32
					);
				}
				else if width {
					root.body.surface.size(
						window.inner_size().width as f32, 
						child_size.height
					);
				}
				else if height {
					root.body.surface.size(
						child_size.width, 
						window.inner_size().height as f32
					);
				}
			},
			IntrinsicSize::Fit{padding} => {
				root.body.surface.size(
					child_size.width + padding as f32, 
					child_size.height + padding as f32
				);
			},
			IntrinsicSize::Fixed(width,height) => {
				root.body.surface.size(width, height);
			}
			_ => {}
		}
	}

	/// Size the children and return their size.
	fn get_constraints(&self,id:WidgetID,max_size:&Size) -> Size {
		let mut max_height:f32 = 0.0;
		let mut max_width:f32 = 0.0;
		
		let node = self.lookup(id).unwrap();

		for (_,edge) in node.children.iter().enumerate(){
			let node = self.lookup(*edge).unwrap();
			match node.body.constraint{
				IntrinsicSize::Fill{width,height} => {
				},
				IntrinsicSize::Fit{padding} => {
					let child_size = self.get_constraints(*edge, max_size);
				},
				IntrinsicSize::Fixed(width,height) => {
					max_height = max_height.max(height);
					max_width = max_width.max(width);
				}
			}
		}

		Size::new(max_width, max_height)
	}

	fn render_widget(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
		id:WidgetID
	) {
		let widget = self.lookup(id).unwrap();
		widget.body.render(display, frame, window, context);

		for (_,edge) in widget.children.iter().enumerate(){
				let child = self.lookup(*edge).unwrap();
				dbg!(&child);
				child.body.render(display, frame, window, context);
		}
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
		self.render_widget(display, frame, window, context, self.root_id);
	}
}
