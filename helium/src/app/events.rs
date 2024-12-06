use std::{collections::VecDeque, fmt::Debug};
use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::{widgets::{Widget, WidgetBody, WidgetId, WidgetState}, Position};

type EventFunction = Box<dyn FnMut()>; 
pub enum Event {
	OnClick(EventFunction),
	OnHover(EventFunction),
}

#[derive(Debug)]
pub struct UserEvent {
	function:Event,
	id:WidgetId
}

impl UserEvent {
	pub fn new(id:WidgetId,f:Event) -> Self{
		Self { 
			function:f,
			id,
		}
	}

	pub fn click(id:WidgetId,f:Event) -> Self{
		Self { 
			function:f,
			id
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

pub struct EventLoop{
	queue:Vec<UserEvent>
}

impl EventLoop {
	pub fn new() -> Self{
		Self { queue: vec![] }
	}

	pub fn push(&mut self, event:UserEvent){
		self.queue.push(event);
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

	pub fn get_type(&self) -> EventType{
		self._type
	}

	pub fn click(id:WidgetId) -> Self{
		Self { 
			widget_id: id, 
			_type: EventType::Click 
		}
	}
}

#[derive(Debug)]
pub struct EventQueue{
	queue:Vec<EventSignal>,
	cursor_pos:Position,
	_loop:Vec<UserEvent>
}

impl EventQueue {
	pub fn new() -> Self{
		Self { 
			queue: Vec::new(),
			cursor_pos:Position::default(),
			_loop:vec![]
		}
	}

	pub fn push(&mut self,event:UserEvent){
		self._loop.push(event);
	}

	pub fn queue(&self) -> &[EventSignal]{
		&self.queue
	}

	/// Get all the events relevant to the current widget by id
	pub fn get_events(&mut self,id:&str) -> Vec<&EventSignal> {
		let events = self.queue.iter().filter(|event|event.id() == &id).collect::<Vec<&EventSignal>>();
		events
	}

	/// Check if the cursor is over the [`Widget`]
	pub fn check_click(&mut self,root_body:&WidgetBody){
		// FIXME it's triggering slightly outside
		let bounds = root_body.surface.get_bounds();
		if bounds.within(&self.cursor_pos){
			self.queue.push(EventSignal::click(root_body.id.clone()));
		}

		if !bounds.within(&self.cursor_pos){
			return;
		}

		for e in &mut self._loop{
			if e.id != root_body.id{continue}
			match &mut e.function {
				Event::OnClick(func) => func(),
				_ => {}
			}
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
						match state {
							winit::event::ElementState::Pressed => {
								self.check_click(root_body);
							},
							winit::event::ElementState::Released => {

							}
						}
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
