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
	}, colour::Colour, layout::{IntrinsicSize, Layout}, surface::{
		image::ImageSurface, rect::RectSurface, text::TextSurface, Surface
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


/// A simple rectangle
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Rect{
	pub width:u32,
	pub height:u32,
	pub colour:Colour
}

impl Rect {
	pub fn new(width:u32,height:u32,colour:Colour) -> Self{
		Self { width, height, colour }
	}
}

impl Widget for Rect {
	fn build(&self) -> WidgetBody {
		let layout = Layout::SingleChild{width:self.width,height:self.height};
		let surface = Box::new(
			RectSurface{ 
				size:Size::new(self.width as f32, self.height as f32),
				colour:self.colour.clone(),
				..Default::default()
			}
		);
		
		WidgetBody{ 
			surface,
			layout,
			children:vec![],
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}


#[derive(Debug)]
pub struct Button{
	pub text:String,
	pub colour:Colour,
	//events:Vec<EventFunction>
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			text:text.into(), 
			colour:Colour::Rgb(255, 255, 255),
			//events:Vec::new()
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self {
		self.colour = colour;
		self
	}

	/* pub fn on_hover(mut self,f:impl Fn() + 'static) -> Self {
		self.events.push(EventFunction::OnHover(Box::new(f)));
		self
	}

	pub fn on_click(mut self,f:impl Fn() + 'static) -> Self{
		self.events.push(EventFunction::OnClick(Box::new(f)));
		self
	} */
}

impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface::new(0.0, 0.0, 200, 70, Colour::Rgb(255, 255, 255))
		);

		let layout = Layout::SingleChild { width: 250, height: 70 };
		// FIXME

		WidgetBody { 
			surface,
			layout,
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}



/// A container [`Widget`] that wraps its child
#[derive(Debug)]
pub struct Container{
	pub padding:u32,
	pub colour:Colour,
	pub child:Box<dyn Widget>
}

impl Container {
	pub fn new(child:impl Widget + 'static) -> Self{
		Container { 
			padding:0, 
			colour:Colour::Rgb(255, 255, 255), 
			child:Box::new(child)
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self{
		self.colour = colour;
		self
	}

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}
}

impl Widget for Container {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface{
				colour:self.colour.clone(),
				..Default::default()
			}
		);
		
		let layout = Layout::Single { padding: 12 };
		let child = self.child.build();

		WidgetBody{
			surface,
			layout,
			children:vec![Box::new(child)],
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		return vec![self.child];
	}
}



#[derive(Debug)]
pub enum FlexDirection {
	Vertical,
	Horizontal
}

#[derive(Debug)]
pub struct Flex{
	order:Vec<u8>,
	direction:FlexDirection,
	children:Vec<Box<dyn Widget>>
}

impl Widget for Flex {
	fn build(&self) -> WidgetBody {
		let children = self.children.iter().map(|widget|{
			Box::new(widget.build())
		}).collect();

		WidgetBody{
			children,
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		self.children
	}
}

/// Simple image widget
#[derive(Debug)]
pub struct Image{
	pub path:String,
	pub width:u32,
	pub height:u32
}

impl Image {
	pub fn new(path:&str,width:u32,height:u32) -> Self{
		Self { path:path.to_owned(), width, height }
	}
}

impl Widget for Image {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			ImageSurface::new(&self.path,self.width as f32,self.height as f32)
		);
		let size = surface.get_size();
		let layout = Layout::SingleChild { width: size.width as u32, height:size.height as u32 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Text{
	pub text:String,
	pub font_size:u8
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			text:text.into(), 
			font_size:16 
		}	
	}

	/// Set the font size
	pub fn font_size(mut self,size:u8) -> Self{
		self.font_size = size;
		self
	}
}

impl Widget for Text {
	fn build(&self) -> WidgetBody {
		// Create the text surface to be rendered
		let textsurface = TextSurface::new(
			self.text.as_str(),
			"#000000" , 
			self.font_size
		);

		let size = textsurface.get_size();
		let surface = Box::new(textsurface);

		let layout = Layout::SingleChild { width: size.width as u32, height: size.height as u32 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}
	
	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}

#[derive(Debug)]
pub struct VStack{
	pub spacing:u32,
	pub padding:u32,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for VStack {
	fn build(&self) -> WidgetBody {
		let layout = Layout::Vertical { 
			spacing:self.spacing, 
			padding:self.padding 
		};

		let children = self.children.iter().map(|widget|{
			Box::new(widget.build())
		}).collect();

		WidgetBody{
			layout,
			children,
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		self.children
	}
}

#[derive(Debug)]

pub struct HStack{
	pub spacing:u32,
	pub padding:u32,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for HStack {
	fn build(&self) -> WidgetBody {
		let layout = Layout::Horizontal  { spacing: self.spacing, padding: self.padding };
		
		let children = self.children.iter().map(|widget|{
			Box::new(widget.build())
		}).collect();

		WidgetBody{
			layout,
			children,
			..Default::default()
		}

	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		self.children
	}
}




