mod rect;
mod container;
mod text;
mod button;
mod circle;
mod vstack;
mod hstack;
use nanoid::nanoid;
pub use rect::Rect;
pub use text::Text;
pub use button::Button;
pub use hstack::HStack;
pub use vstack::VStack;
pub use container::Container;
pub use circle::Circle;
use crate::{
	app::AppState, 
	layout::{BlockLayout, BoxContraints, IntrinsicSize, Layout, WidgetSize}, 
	surface::{
		rect::RectSurface, Surface
	}, 
};
use helium_core::position::Position;
use helium_core::size::Size;

pub type WidgetId = String; // FIXME Redundant type

/// The trait that all widgets must implement. Each `widget` must implement build function
/// which returns a [`WidgetBody`]. `widgetbodies` are objects that hold information about 
/// the widget.
pub trait Widget{
	// I've changed this between &self and self, a couple times and my conclusion is 
	// just keep it as &self forever, it makes it way easier to compose multiple sub-widgets.

	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> WidgetBody;
}

// TODO maybe implement iter
/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{ // TODO this changes a lot so make these fields private
	pub id:WidgetId,
	/// A label for debugging purposes
	pub label:Option<String>,
	pub surface:Box<dyn Surface>,
	pub layout:Box<dyn Layout>,
	pub children:Vec<Box<WidgetBody>>,
	 // TODO move this to the layout
}

impl WidgetBody {
	pub fn new() -> Self{
		Self::default()	
	}

	pub fn label(mut self,label:&str) -> Self {
		self.label = Some(label.to_owned());
		self
	}

	pub fn surface(mut self,surface:Box<dyn Surface>) -> Self{
		self.surface = surface;
		self
	}

	pub fn layout(mut self,layout:impl Layout + 'static) -> Self{
		self.layout = Box::new(layout);
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
		// Maybe return the sizes so instead of passing mutable state
		// FIXME this is running for every widget with the window size
		self.arrange(*window_size);
		
		// Draw the parent then the children to the screen
		self.surface.draw(render_pass, context,state);
		self.children.iter_mut().for_each(|child|{
			child.render(render_pass, state);
		});
	}

	pub fn arrange(&mut self,window_size:Size){
		let position = Position::new(
			self.surface.get_position().x, 
			self.surface.get_position().y
		);

		// Arrange the children and return min size
		let size = self.layout.compute_layout(&mut self.children,window_size,position);
	}
}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());

		Self { 
			id:nanoid!(),
			surface, 
			label:None,
			layout:Box::new(BlockLayout::new(0)), 
			children:vec![], 
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

		
	};
}