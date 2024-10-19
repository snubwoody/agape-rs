mod rect;
mod stack;
mod container;
mod text;
mod button;
pub use rect::Rect;
pub use text::Text;
pub use button::Button;
pub use stack::Stack;

use std::fmt::Debug;
use crate::{
	app::{AppState, RenderContext}, 
	layout::{IntrinsicSize, Layout, WidgetSize}, 
	surface::{
		rect::RectSurface, Surface
	}, utils::{Position, Size}, 
};


/// The trait that all widgets must implement.
pub trait Widget:Debug{
	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> WidgetBody;

	fn run_events(&mut self,event:winit::event::WindowEvent){}

	/// Get the children and consume the [`Widget`], 
	/// since this is the last step before the widget
	/// is turned to a [`WidgetBody`].  
	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>>;
}


/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{
	pub surface:Box<dyn Surface>,
	pub layout:Layout,
	pub children:Vec<Box<WidgetBody>>,
	pub intrinsic_size:IntrinsicSize
	//pub events:Vec<EventFunction>
}

impl WidgetBody {
	/// Draw the [`WidgetBody`] to the screen.
	pub fn render(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		state: &AppState
	) {
		let window_size = &state.size;
		let context = &state.context;
		
		// Arrange the children
		let size = self.layout.arrange_widgets(
			&mut self.children,
			Size::new(window_size.width, window_size.height),
			Position::new(
				self.surface.get_position().x, 
				self.surface.get_position().y
			)
		);

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
		
		// Draw the parent then the children to the screen
		self.surface.draw(render_pass, context,state);
		self.children.iter_mut().for_each(|child|{
			child.surface.draw(render_pass, context,state);
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
			intrinsic_size: Default::default()
		}
	}
}