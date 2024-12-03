use std::{collections::VecDeque, fmt::Debug};
use winit::{event::{ElementState, MouseButton, WindowEvent}, event_loop::EventLoopProxy};
use crate::{widgets::{Widget, WidgetBody, WidgetId, WidgetState}, Position};

pub enum Event {
	OnClick(Box<dyn FnMut()>),
	OnHover(Box<dyn FnMut()>),
}

pub enum UserEvent {
	OnClick(Box<dyn FnMut()>)
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

#[derive(Debug,Clone, Copy,PartialEq)]
pub enum EventType {
	Click,
	Hover
}

#[derive(Debug)]
pub struct EventSignal{
	widget_id:WidgetId,
	_type:EventType
}

impl EventSignal {
	pub fn click(id:WidgetId) -> Self{
		Self { 
			widget_id: id, 
			_type: EventType::Click 
		}
	}
}

#[derive(Debug)]
pub enum Signal{
	Hover(WidgetId),
	Click(WidgetId)
}

pub(crate) struct EventQueue{
	queue:VecDeque<EventSignal>,
	cursor_pos:Position,
}

impl EventQueue {
	pub fn new() -> Self{
		Self { 
			queue: VecDeque::new(),
			cursor_pos:Position::default() 
		}
	}

	pub fn dispatch(&self,widget_body:&mut WidgetBody){
		for event in self.queue.iter(){
			if event.widget_id == widget_body.id{
				widget_body.run_events(event._type);
			}
		}
	}

	pub fn run_events(
		&self,
		root_widget:&mut Box<dyn Widget>,
		root_body:&mut WidgetBody
	) {
		self.dispatch(root_body);
	}

	/// Check if the cursor is over the [`Widget`]
	pub fn check_click(&mut self,root_body:&WidgetBody){
		// FIXME it's triggering slightly outside
		let bounds = root_body.surface.get_bounds();
		if bounds.within(&self.cursor_pos){
			self.queue.push_back(EventSignal::click(root_body.id.clone()));
		}
	}

	pub fn handle_events(
		&mut self,
		event:&winit::event::WindowEvent,
		root_body:&WidgetBody
	) {
		match event {
			WindowEvent::MouseInput { state, button,.. } => {
				match button {
					winit::event::MouseButton::Left => {
						self.check_click(root_body);
					},
					_ => {}
				}
			},
			WindowEvent::CursorMoved { position,.. } => {
				// Update the cursor position every time it moves
				self.cursor_pos = Position::from(*position);
			}
			_ => {}
		}
	}
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

