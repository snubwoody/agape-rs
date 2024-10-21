use std::fmt::Debug;
use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::{utils::Position, widgets::{Widget, WidgetState}};

pub enum Event<State> {
	/// Occurs when the mouse button is clicked
	/// this event only fires when the mouse button 
	/// is released.
	OnClick(Box<dyn FnMut(&mut State)>),

	/// Occurs when the mouse button is pressed down.
	OnPress(Box<dyn FnMut(&mut State)>),

	/// Occurs when the mouse position is 
	/// within the [`Bounds`] of the [`Widget`].
	OnHover(Box<dyn FnMut(&mut State)>),
}


impl<State> Debug for Event<State> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			Self::OnClick(_) => {
				f.debug_tuple("OnClick()").finish()
			},
			Self::OnPress(_) => {
				f.debug_tuple("OnPress()").finish()
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
		match event {
			WindowEvent::CursorMoved { position,.. } => {
				self.cursor_pos = position.clone().into();
				//root_widget.handle_hover(self.cursor_pos);
			},
			WindowEvent::MouseInput { state, button,.. } => {
				match state {
					ElementState::Pressed => self.mouse_button_down = true,
					ElementState::Released => self.mouse_button_down = false
				}

				match button {
					MouseButton::Left => {
						match state {
							ElementState::Pressed => {
								//root_widget.handle_press(self.cursor_pos);
								root_widget.change_state(WidgetState::Pressed);
							},
							ElementState::Released => {
								//root_widget.handle_click(self.cursor_pos);
								root_widget.change_state(WidgetState::Default);	
							}
						}
					},
					_ => {}
				}
			}
			_ => {}
		}
	}

}

