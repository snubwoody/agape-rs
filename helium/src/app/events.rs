use std::{collections::VecDeque, fmt::Debug};
use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::{widgets::{Widget, WidgetBody, WidgetId, WidgetState}, Position};

pub enum Event {
	OnClick(Box<dyn FnMut()>),
	OnHover(Box<dyn FnMut()>),
}

type EventFunction = Box<dyn FnMut()>; 
pub struct UserEvent {
	function:EventFunction,
	trigger:EventType
}

impl UserEvent {
	pub fn new(f:impl FnMut() + 'static,trigger:EventType) -> Self{
		Self { 
			function:Box::new(f),
			trigger 
		}
	}

	pub fn click(f:impl FnMut() + 'static) -> Self{
		Self { 
			function:Box::new(f),
			trigger:EventType::Click 
		}
	}
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
	pub fn id(&self) -> &WidgetId{
		&self.widget_id
	}
	pub fn click(id:WidgetId) -> Self{
		Self { 
			widget_id: id, 
			_type: EventType::Click 
		}
	}
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

	pub fn dispatch(&self,widget:&mut Box<dyn Widget>){
		for event in self.queue.iter(){
			widget.run_events(event);
		}
	}

	pub fn run_events(
		&self,
		root_widget:&mut Box<dyn Widget>,
		root_body:&mut WidgetBody
	) {
		self.dispatch(root_widget);
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
