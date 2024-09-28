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
	nodes:HashMap<WidgetID,Node>,
	root_id:WidgetID
}

impl WidgetTree {
	pub fn new() -> Self{
		Self{
			nodes:HashMap::new(),
			root_id:0
		}
	}

	pub fn build(&mut self,widget:impl Widget + 'static) {
		let root = widget.build();
		let children = self.add_edges(Box::new(widget),0);
		self.nodes.insert(0,Node{
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

			self.nodes.insert(child_id, child);
			
		}
		edges
	}

	fn size_pass(&mut self,window:&Window){
		let root = self.nodes.get_mut(&self.root_id).unwrap();
		match root.body.constraint.width {
			WidgetSize::Fill => {
				root.body.surface.width(
					window.inner_size().width as f32, 
				);
			},
			WidgetSize::Fit(padding) => {
				
			}
			_ => {}
		}
	}

	fn calculate_size(&mut self,id:WidgetID,constraint:Size) -> Size{
		Size::new(0.0, 0.0)
	}

	fn render_widget(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
		id:WidgetID
	) {
		let widget = self.nodes.get(&id).unwrap();
		widget.body.render(display, frame, window, context);

		// Render the widgets recursively
		for (_,edge) in widget.children.iter().enumerate(){
			let child = self.nodes.get(edge).unwrap();
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
		self.render_widget(display, frame, window, context, self.root_id);
	}
}
