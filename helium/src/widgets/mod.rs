mod rect;
mod stack;
mod container;
mod text;
mod button;
pub use rect::Rect;
pub use text::Text;
pub use button::Button;
pub use stack::Stack;
use winit::event::WindowEvent;

use std::fmt::Debug;
use crate::{
	app::{AppState, RenderContext}, 
	layout::{IntrinsicSize, Layout, WidgetSize}, 
	surface::{
		rect::RectSurface, Surface
	}, utils::{Position, Size}, 
};


/// This is (hopefully) a temporary macro, to reduce code
/// duplication when creating [`Widget`]'s.
#[macro_export]
macro_rules! impl_events {
	($name:ty) => {
		pub fn on_hover(mut self, event: impl FnMut(&mut $name) + 'static ) -> Self {
			self.events.push(Event::OnHover(Box::new(event)));
			self
		}
	
		pub fn on_click(mut self, event: impl FnMut(&mut $name) + 'static ) -> Self {
			self.events.push(Event::OnClick(Box::new(event)));
			self
		}
	
		pub fn on_press(mut self, event: impl FnMut(&mut $name) + 'static ) -> Self {
			self.events.push(Event::OnPress(Box::new(event)));
			self
		}
	};
}

/// Implement the interactive functions of the [`Widget`] trait,
/// the code is usually the same, so this is to reduce code duplication
/// and frustration.
#[macro_export]
macro_rules! impl_interative {
	() => {
		fn handle_hover(&mut self,cursor_pos:crate::utils::Position) {
			let body = self.build();
			let bounds = body.surface.get_bounds();
			let mut state = self.snapshot();
	
			if bounds.within(&cursor_pos){
				for event in self.events.iter_mut(){
					match event {
						crate::app::events::Event::OnHover(func) => func(&mut state),
						_ => {}
					}
				}
			}
			self.update(&state);
		}
	
		fn handle_click(&mut self,cursor_pos:crate::utils::Position) {
			let body = self.build();
			let bounds = body.surface.get_bounds();
			let mut state = self.snapshot();
	
			if bounds.within(&cursor_pos){
				for event in self.events.iter_mut(){
					match event {
						crate::app::events::Event::OnClick(func) => func(&mut state),
						_ => {}
					}
				}
			}
			self.update(&state);
		}

		fn handle_press(&mut self,cursor_pos:crate::utils::Position) {
			let body = self.build();
			let bounds = body.surface.get_bounds();
			let mut state = self.snapshot();
	
			if bounds.within(&cursor_pos){
				for event in self.events.iter_mut(){
					match event {
						crate::app::events::Event::OnPress(func) => func(&mut state),
						_ => {}
					}
				}
			}
			self.update(&state);
		}
	};
}


/// The trait that all widgets must implement.
pub trait Widget:Debug{
	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> WidgetBody;
  
	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {vec![]}
	
	fn handle_hover(&mut self,cursor_pos:Position);
	fn handle_click(&mut self,cursor_pos:Position);
	fn handle_press(&mut self,cursor_pos:Position);
}


/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{
	pub surface:Box<dyn Surface>,
	pub layout:Layout,
	pub children:Vec<Box<WidgetBody>>,
	pub intrinsic_size:IntrinsicSize
	//pub events:Vec<Event>
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