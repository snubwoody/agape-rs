use crystal::{Layout, Position};
use helium_core::position::Bounds;
use winit::event::WindowEvent;
use crate::widgets::Widget;


pub enum EventFn {
    OnHover(Box<dyn FnMut()>),
    OnClick(Box<dyn FnMut()>),
}

impl EventFn {
    pub fn run_hover(&mut self) {
        match self {
            Self::OnHover(func) => (func)(),
            _ => {},
        }
    }
 
    pub fn run_click(&mut self) {
        match self {
            Self::OnClick(func) => (func)(),
            _ => {},
        }
    }
}

#[derive(Debug,Default,PartialEq,Eq, PartialOrd,Ord,Clone, Copy,Hash)]
enum ElementState{
	#[default]
	Default,
	Hovered,
	Clicked
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Event {
    #[default]
    Hover,
	Clicked
}

/// Describes the state of a [`Widget`]
#[derive(Debug,Clone, PartialEq, Eq,PartialOrd,Ord)]
struct Element {
	id:String,
	previous_state:ElementState,
	state:ElementState,
}

impl Element {
	fn new(id:&str) -> Self{
		Self{
			id:String::from(id),
			previous_state:ElementState::Default,
			state:ElementState::Default,
		}
	}

	/// Set the element state to whatever it was previously
	fn roll_back(&mut self){
		self.state = self.previous_state;
	}
	
	/// Set the element state to `ElementState::Default`
	fn default(&mut self){
		self.previous_state = self.state;
		self.state = ElementState::Default;
	}

	/// Set the element state to `ElementState::Clicked`
	fn click(&mut self){
		self.previous_state = self.state;
		self.state = ElementState::Clicked;
	}

	/// Set the element state to `ElementState::Hovered`
	fn hover(&mut self){
		self.previous_state = self.state;
		self.state = ElementState::Hovered;
	}
}

#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct Notify {
    id: String,
    event: Event,
}

impl Notify {
	pub fn id(&self) -> &str {
		&self.id
	}

    pub fn new(id: &str, event: Event) -> Self {
        Self {
            id: id.to_string(),
            event,
        }
    }

    pub fn hover(id: &str) -> Self {
        Self {
            id: id.to_string(),
            event: Event::Hover,
        }
    }

    pub fn click(id: &str) -> Self {
        Self {
            id: id.to_string(),
            event: Event::Clicked,
        }
    }

    pub fn event(&self) -> Event {
        self.event
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct EventManager {
    mouse_pos: Position,
	elements: Vec<Element>,
	notifications: Vec<Notify>
}

impl EventManager {
    pub fn new(layout: &dyn Layout) -> Self {
		let elements:Vec<Element> = layout.iter().map(|l|Element::new(l.id())).collect();
        
		Self{
			elements,
			mouse_pos:Position::default(),
			notifications:vec![]
		}
    }

	/// Get a reference to an [`Element`] by it's `id`
	fn element(&self,id:&str) -> Option<&Element>{
		self.elements.iter().find(|e|e.id == id)
	}

	/// Get a `&mut` to an [`Element`] by it's `id`
	fn element_mut(&mut self,id:&str) -> Option<&mut Element>{
		self.elements.iter_mut().find(|e|e.id == id)
	}

	fn process_hover(&mut self,layout: &dyn Layout){
		let bounds = Bounds::new(layout.position(), layout.size());
		let mouse_pos = self.mouse_pos;
		let element = self.elements.iter_mut().find(|e|e.id == layout.id()).unwrap();

		if bounds.within(&mouse_pos){
			match element.state {
				ElementState::Default => {
					self.notifications.push(Notify::hover(layout.id()));
					element.hover();
				},
				_ => {}
			}
		}else {
			element.default();
			return;
		}
	}

	fn process_mouse(
		&mut self,
		layout: &dyn Layout,
		state:&winit::event::ElementState,
		button:&winit::event::MouseButton
	){
		let element = self.elements.iter_mut().find(|e|e.id == layout.id()).unwrap();
		// TODO use right click only
		match state {
			&winit::event::ElementState::Pressed => {
				match element.state {
					ElementState::Default => {},
					ElementState::Hovered => {
						self.notifications.push(Notify::click(layout.id()));
						element.click();
					},
					ElementState::Clicked => {}
				}
			}
			&winit::event::ElementState::Released => {
				// Not sure about this
				element.roll_back();
			}
		}
		
	}

	/// Process the incoming `WindowEvent` and dispatch events to [`Widget`]'s
    pub fn process(
        &mut self,
        event: &winit::event::WindowEvent,
        layout: &dyn Layout,
    ){
		//dbg!(&self.elements);
        match event {
			WindowEvent::CursorMoved {position,..} => {
				self.mouse_pos = (*position).into();
                for layout in layout.iter() {
					self.process_hover(layout);
                }
            },
            WindowEvent::MouseInput {state,button,..} => {
				for layout in layout.iter() {
					self.process_mouse(layout,state, button);
                }
			},
            _ => {}
        }
    }

	pub fn notify(&mut self,widget: &dyn Widget){
		for notif in self.notifications.drain(..){
			let widget = widget.get(notif.id()).unwrap();
			widget.notify(&notif);
		}
	}
}


#[cfg(test)]
mod test{
	use crystal::{EmptyLayout, Size};
	use winit::{
		dpi::PhysicalPosition, 
		event::{DeviceId, ElementState as WinitElementState, MouseButton}
	};
	use super::*;

	#[test]
	fn mouse_position_updates(){
		let mut events = EventManager::new(&EmptyLayout::default());
		
		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(50.0, 60.0);
		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &EmptyLayout::default());
		assert_eq!(events.mouse_pos,position.into())
	}

	#[test]
	fn hover_event(){
		let mut layout = EmptyLayout::default();
		layout.id = String::from("id");
		layout.position = Position::new(50.0, 50.0);
		layout.size = Size::new(100.0, 100.0);
		let mut events = EventManager::new(&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(92.23, 63.2);

		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &layout);

		assert!(events.notifications.contains(&Notify::hover("id")))
	}

	#[test]
	fn no_duplicate_hover_events(){
		let mut layout = EmptyLayout::default();
		layout.position = Position::new(50.0, 50.0);
		layout.size = Size::new(100.0, 100.0);
		let mut events = EventManager::new(&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(92.23, 63.2);

		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &layout);
		events.process(&cursor_event, &layout);
		events.process(&cursor_event, &layout);
		events.process(&cursor_event, &layout);

		assert!(events.notifications.len() == 1)
	}

	#[test]
	fn click_event(){
		let layout = EmptyLayout::default();
		let mut events = EventManager::new(&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let click_event = WindowEvent::MouseInput { 
			device_id, 
			state: WinitElementState::Pressed, 
			button: MouseButton::Left
		};

		events.elements[0].state = ElementState::Hovered;
		events.process(&click_event, &layout);

		assert!(events.notifications.contains(&Notify::click(layout.id())))
	}

	#[test]
	fn hover_state(){
		let mut layout = EmptyLayout::default();
		layout.size = Size::new(500.0, 500.0);
		let mut events = EventManager::new(&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(92.23, 63.2);

		let cursor_event = WindowEvent::CursorMoved {device_id,position};
		events.process(&cursor_event, &layout);

		assert_eq!(events.elements[0].state,ElementState::Hovered);
	}

	#[test]
	fn click_element_state(){
		let layout = EmptyLayout::default();
		let mut events = EventManager::new(&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let click_event = WindowEvent::MouseInput { 
			device_id, 
			state: WinitElementState::Pressed, 
			button: MouseButton::Left
		};
		events.elements[0].state = ElementState::Hovered;
		events.process(&click_event, &layout);
		assert_eq!(events.elements[0].state,ElementState::Clicked);
		
		let click_event = WindowEvent::MouseInput { 
			device_id, 
			state: WinitElementState::Released, 
			button: MouseButton::Left
		};
		events.process(&click_event, &layout);
		assert_eq!(events.elements[0].state,ElementState::Hovered);
	}
}