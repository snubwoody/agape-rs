use std::fmt::Debug;
use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::{utils::Position, widgets::Widget};

/// A [`Widget`] that can react to events
pub trait Interactive{
	fn handle_hover(&self,cursor_pos:Position){

	}
}

pub enum Event<State> {
	OnClick(Box<dyn FnMut(&mut State)>),
	OnHover(Box<dyn FnMut(&mut State)>),
}

impl<State> Event<State> {
	pub fn run(&mut self,widget:&mut State) {
		match self{
			Self::OnClick(func) => func(widget),
			Self::OnHover(func) => func(widget),
		}
	}
}

impl<State> Debug for Event<State> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			Self::OnClick(_) => {
				f.debug_tuple("OnClick()").finish()
			},
			Self::OnHover(_) => {
				f.debug_tuple("OnHover()").finish()
			},
		}
	}
}

/// Handles all widget events and stores
/// useful attributes such as the cursor
/// position and the delta position.
pub struct EventHandler{
	cursor_pos: Position,
	delta_pos: Position,
	mouse_button_down: bool
}

impl EventHandler {
	pub fn new() -> Self{
		Self { 
			cursor_pos: Position::default(),
			delta_pos:Position::default(),
			mouse_button_down:false 
		}
	}

	pub fn handle_events(&mut self,event:&winit::event::WindowEvent,root_widget:&mut Box<dyn Widget>){
		root_widget.run_events(event);
		match event {
			WindowEvent::CursorMoved { position,.. } => {
				self.cursor_pos = position.clone().into();
			},
			WindowEvent::MouseInput { state, button,.. } => {
				match state {
					ElementState::Pressed => self.mouse_button_down = true,
					ElementState::Released => self.mouse_button_down = false
				}

				match button {
					MouseButton::Left => {
						if self.mouse_button_down{
							dbg!("Mouse was pressed");
						}
					},
					_ => {}
				}
			}
			_ => {}
		}
	}

}

