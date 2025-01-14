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

struct EventBody {
    mouse_over: bool,
    mouse_down: bool,
}

pub struct Notification {
    id: String,
    event: Event,
}

impl Notification {
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct EventManager {
    mouse_pos: Position,
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle(
        &mut self,
        event: &winit::event::WindowEvent,
        layout: &dyn Layout,
    ) -> Vec<Notification> {
        let mut notifications = vec![];

        match event {
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                self.mouse_pos = (*position).into();
                for layout in layout.iter() {
                    let bounds = Bounds::new(layout.position(), layout.size());
                    if bounds.within(&self.mouse_pos) {
                        notifications.push(Notification::hover(layout.id()));
                    }
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {}
            _ => {}
        }

        notifications
    }

    fn notify(&self, widget: &mut dyn Widget) {}
}
