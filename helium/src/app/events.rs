use std::fmt::Debug;
use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::{utils::Position, widgets::{Widget, WidgetBody, WidgetState}};

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
}

impl EventHandler {
	pub fn new() -> Self{
		Self { 
			cursor_pos: Position::default(),
		}
	}

	pub fn handle_events(
		&mut self,
		event:&winit::event::WindowEvent,
		root_widget:&mut Box<dyn Widget>,
		root_body:&mut WidgetBody
	) {
		let mut signals = vec![];
		let bounds = root_body.surface.get_bounds();
		let previous_state = root_body.state;

		match event {
			WindowEvent::CursorMoved { position,.. } => {
				self.cursor_pos = position.clone().into(); // Calling clone this much might be expensive
				if bounds.within(&self.cursor_pos){
					root_body.state = WidgetState::Hovered;
					match &previous_state { // Only run the on_hover once
						&WidgetState::Default => {
							signals.push(Signal::Hover(root_body.id.clone()));
						},
						_ => {}
					}
				} 
				else {
					root_body.state = WidgetState::Default;
				}
			}, 
			//TODO add on_click, on_right_click, and on_all_click and pass in the mouse button (re-export types)
			WindowEvent::MouseInput { state, button,.. } => {				
				match button {
					MouseButton::Left => {
						match state {
							ElementState::Pressed => {
								root_body.state = WidgetState::Clicked;
								if bounds.within(&self.cursor_pos){
									signals.push(Signal::Click(root_body.id.clone()));
								}
							},
							ElementState::Released => {
								root_body.state = WidgetState::Default
							}
						}
					},
					_ => {}
				}
			}
			_ => {}
		}

		for signal in signals.iter(){
			root_widget.process_signal(signal);
		}
	}
}

