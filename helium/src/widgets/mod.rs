mod rect;
mod stack;
mod container;
mod text;
mod button;
use nanoid::nanoid;
pub use rect::Rect;
pub use text::Text;
pub use button::Button;
pub use stack::Stack;
pub use container::Container;
use crate::{
	app::AppState, 
	layout::{IntrinsicSize, Layout, WidgetSize}, 
	surface::{
		rect::RectSurface, Surface
	}, 
};
use helium_core::position::Position;
use helium_core::size::Size;

pub type WidgetId = String;

/// The trait that all widgets must implement.
pub trait Widget{
	// I've changed this between &self and self, a couple times and my conclusion is 
	// just keep it as &self forever, it makes it way easier to compose multiple sub-widgets.

	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> WidgetBody;
}

/// The current state of the widget
#[derive(Debug,Default,Clone,Copy,PartialEq,Eq)]
pub enum WidgetState{
	#[default]
	Default,
	Hovered,
	Clicked
}

// TODO maybe implement iter
/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{ // TODO this changes a lot so make these fields private
	pub id:WidgetId,
	pub surface:Box<dyn Surface>,
	pub layout:Layout,
	pub children:Vec<Box<WidgetBody>>,
	pub intrinsic_size:IntrinsicSize, // TODO move this to the layout
	pub state:WidgetState,
}

impl WidgetBody {
	pub fn new() -> Self{
		Self::default()	
	}

	pub fn surface(mut self,surface:Box<dyn Surface>) -> Self{
		self.surface = surface;
		self
	}

	pub fn layout(mut self,layout:Layout) -> Self{
		self.layout = layout;
		self
	}

	pub fn add_child(mut self,child:WidgetBody) -> Self{
		self.children.push(Box::new(child));
		self
	}

	pub fn add_children(mut self,children:Vec<WidgetBody>) -> Self{
		for child in children{
			self.children.push(Box::new(child));
		};
		self
	}

	pub fn intrinsic_size(mut self,intrinsic_size:IntrinsicSize) -> Self{
		self.intrinsic_size = intrinsic_size;
		self
	}

	/// Draw the [`WidgetBody`] to the screen.
	pub(crate) fn render(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		state: &AppState
	) {
		let window_size = &state.size;
		let context = &state.context;

		// TODO I think I should change this so that ALL
		// of the layout is handled by the Layout struct
		self.arrange(*window_size);
		
		// Draw the parent then the children to the screen
		self.surface.draw(render_pass, context,state);
		self.children.iter_mut().for_each(|child|{
			child.surface.draw(render_pass, context,state);
		});
	}

	pub(crate) fn arrange(&mut self,window_size:Size){
		let position = Position::new(
			self.surface.get_position().x, 
			self.surface.get_position().y
		);

		// Arrange the children
		let size = self.layout.compute_layout(&mut self.children,window_size,position);

		// Set the size of the root widget
		match self.intrinsic_size.width {
			WidgetSize::Fill => {
				self.surface.width(window_size.width as f32);
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
				self.surface.height(window_size.height as f32);
			},
			WidgetSize::Fit => {
				self.surface.height(size.height);
			},
			WidgetSize::Fixed(size) => {
				self.surface.height(size);
			}
		}
	}
}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());
		let layout = Layout::default();

		Self { 
			id:nanoid!(),
			surface, 
			layout, 
			children:vec![], 
			intrinsic_size: Default::default(),
			state: WidgetState::default(),
		}
	}
}


/// Implement common styling attributes
#[macro_export]
macro_rules! impl_style {
	() => {
		/// Change the [`Color`] of a [`Widget`].
		pub fn color(mut self,color:crate::Color) -> Self{
			self.color = color;
			self
		} 

		pub fn spacing(mut self, spacing: u32) -> Self {
			self.layout = self.layout.spacing(spacing);
			self
		}
	
		pub fn padding(mut self,padding:u32) -> Self{
			self.layout = self.layout.padding(padding);
			self
		}
	};
}