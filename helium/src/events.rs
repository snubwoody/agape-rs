use crystal::{Layout, Position};
use helium_core::position::Bounds;
use winit::event::WindowEvent;
use crate::widgets::Widget;

trait Interactive{
	fn on_click();
	fn on_hover();
	fn while_hover();
	fn while_click();
}

pub enum EventFn {
    OnHover(Box<dyn FnMut()>),
}

impl EventFn {
    pub fn run(&mut self) {
        match self {
            Self::OnHover(func) => (func)(),
        }
    }
}

enum State {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Event {
    #[default]
    Hover,
}

/// Describes the state of a [`Widget`]
#[derive(Debug,Clone, PartialEq, Eq,PartialOrd,Ord)]
struct Element {
	id:String,
    mouse_over: bool,
    mouse_down: bool,
}

impl Element {
	pub fn new(id:&str) -> Self{
		Self{
			id:String::from(id),
			mouse_down:false,
			mouse_over:false
		}
	}
}

#[derive(Debug,Clone,Default,PartialEq, PartialOrd)]
pub struct Notif {
    id: String,
    event: Event,
}

impl Notif {
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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn event(&self) -> Event {
        self.event
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd,Default)]
pub struct EventManager {
    mouse_pos: Position,
	elements: Vec<Element>,
	notifications:Vec<Notif>
}

impl EventManager {
    pub fn new(widget: &dyn Widget) -> Self {
		let elements:Vec<Element> = widget.iter().map(|w|Element::new(w.id())).collect();
        
		Self{
			elements,
			mouse_pos:Position::default(),
			notifications:vec![]
		}
    }

	/// Get an [`Element`] by it's `id`
	fn element(&self,id:&str) -> Option<&Element>{
		self.elements.iter().find(|e|e.id == id)
	}

	fn process_hover(&mut self,layout: &dyn Layout){
		let bounds = Bounds::new(layout.position(), layout.size());
		if bounds.within(&self.mouse_pos) {
			self.notifications.push(Notif::hover(layout.id()));
		}
	}

	/// Process the incoming `WindowEvent` and dispatch events to [`Widget`]'s
    pub fn process(
        &mut self,
        event: &winit::event::WindowEvent,
        layout: &dyn Layout,
    ){
        match event {
            WindowEvent::CursorMoved {position,..} => {
                self.mouse_pos = (*position).into();
                for layout in layout.iter() {
					self.process_hover(*layout);
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {}
            _ => {}
        }
    }

	pub fn notify(&mut self,widget: &dyn Widget){
		for notif in self.notifications.drain(..){
			if let Some(widget) = widget.get(notif.id()){
				widget.notify(&notif);
			}
		}
	}
}


#[cfg(test)]
mod test{
	use crystal::{EmptyLayout, Size};
	use winit::{dpi::PhysicalPosition, event::DeviceId};
	use crate::widgets::Text;
	use super::*;

	#[test]
	fn mouse_position_updates(){
		let mut events = EventManager::default();
		
		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(50.0, 60.0);
		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &EmptyLayout::default());
		assert_eq!(events.mouse_pos,position.into())
	}

	#[test]
	fn hover_event(){
		let mut events = EventManager::new(&Text::new(""));
		let mut layout = EmptyLayout::default();
		layout.id = String::from("id");
		layout.position = Position::new(50.0, 50.0);
		layout.size = Size::new(100.0, 100.0);

		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(92.23, 63.2);

		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &layout);

		assert!(events.notifications.contains(&Notif::hover("id")))
	}

	#[test]
	fn no_duplicate_hover_events(){
		let text = Text::new("");

		let mut events = EventManager::new(&text);
		let mut layout = EmptyLayout::default();
		layout.id = String::from(text.id());
		layout.position = Position::new(50.0, 50.0);
		layout.size = Size::new(100.0, 100.0);

		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(92.23, 63.2);

		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &layout);
		events.process(&cursor_event, &layout);
		events.process(&cursor_event, &layout);
		events.process(&cursor_event, &layout);

		dbg!(&events);

		assert!(events.notifications.len() == 1)
	}
}