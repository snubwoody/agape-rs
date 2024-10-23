use std::fmt::Debug;

use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::{utils::Position, widgets::{Widget, WidgetBody}};

pub enum Event {
	OnClick(Box<dyn FnMut()>),
	OnHover(Box<dyn FnMut()>),
}

impl Debug for Event {
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

#[derive(Debug)]
pub enum Signal{
	Hover(String),
	Click(String)
}


/// Handles all widget events and stores useful attributes such 
/// as the cursor position and the delta position.
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

	pub fn handle_events(
		&mut self,
		event:&winit::event::WindowEvent,
		root_widget:&mut Box<dyn Widget>,
		root_body:&WidgetBody
	) {
		let mut signals = vec![];

		match event {
			WindowEvent::CursorMoved { position,.. } => {
				self.cursor_pos = position.clone().into();
			},
			WindowEvent::MouseInput { state, button,.. } => {
				let bounds = root_body.surface.get_bounds();
				
				
				if bounds.within(&self.cursor_pos){
					signals.push(Signal::Click(root_body.id.clone()));
				}
		
				
				match state {
					ElementState::Pressed => self.mouse_button_down = true,
					ElementState::Released => self.mouse_button_down = false
				}

				match button {
					MouseButton::Left => {
						match state {
							ElementState::Pressed => {
								//root_widget.change_state(WidgetState::Pressed);
							},
							ElementState::Released => {
								//root_widget.change_state(WidgetState::Default);	
							}
						}
					},
					_ => {}
				}
			}
			_ => {}
		}
		
		for signal in signals.iter(){
			root_widget.parse_signal(signal);
		}
	}
}

