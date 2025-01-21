use crystal::{Layout, Position};
use helium_core::position::Bounds;
use std::fmt::Debug;
use winit::event::WindowEvent;

/// Stores callback functions for [`Widget`]'s
pub struct EventContext {
    callbacks: Vec<EventFn>,
}

impl EventContext {
    pub fn new() -> Self {
        Self { callbacks: vec![] }
    }

    pub fn add(&mut self, callback: EventFn) {
        self.callbacks.push(callback);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Shift,
    Enter,
    Ctrl,
    Space,
    Tab,
    CapsLock,
    Fn,
    Alt,
    Super,
    Char(char),
}

pub enum EventFn {
    OnHover(String, Box<dyn FnMut()>),
    OnClick(String, Box<dyn FnMut()>),
    OnKey(String, Box<dyn FnMut(Key)>),
}

impl EventFn {
    pub fn hover(id: &str, f: impl FnMut() + 'static) -> Self {
        Self::OnHover(id.to_string(), Box::new(f))
    }

    pub fn click(id: &str, f: impl FnMut() + 'static) -> Self {
        Self::OnClick(id.to_string(), Box::new(f))
    }

    pub fn key(id: &str, f: impl FnMut(Key) + 'static) -> Self {
        Self::OnKey(id.to_string(), Box::new(f))
    }

    pub fn run_key(&mut self, widget_id: &str, key: Key) {
        match self {
            Self::OnKey(id, func) => {
                if id == widget_id {
                    (func)(key)
                }
            }
            _ => {}
        }
    }

    pub fn run_hover(&mut self, widget_id: &str) {
        match self {
            Self::OnHover(id, func) => {
                if id == widget_id {
                    (func)()
                }
            }
            _ => {}
        }
    }

    pub fn run_click(&mut self, widget_id: &str) {
        match self {
            Self::OnClick(id, func) => {
                if id == widget_id {
                    (func)()
                }
            }
            _ => {}
        }
    }
}

impl Debug for EventFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnClick(id, _) => f.debug_tuple(format!("OnClick({id},_)").as_str()).finish(),
            Self::OnHover(id, _) => f.debug_tuple(format!("OnHover({id},_)").as_str()).finish(),
            Self::OnKey(id, _) => f.debug_tuple(format!("OnKey({id},_)").as_str()).finish(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum ElementState {
    #[default]
    Default,
    Hovered,
    // Maybe add mouse button? or just add RightClicked
    Clicked,
}

/// Describes the state of a [`Widget`]
#[derive(Debug)]
pub struct Element {
    id: String, // TODO add size and position
    previous_state: ElementState,
    state: ElementState,
}

impl Element {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            previous_state: ElementState::Default,
            state: ElementState::Default,
        }
    }

    /// Set the element state to whatever it was previously
    fn roll_back(&mut self) {
        self.state = self.previous_state;
    }

    /// Set the element state to `ElementState::Default`
    fn default(&mut self) {
        self.previous_state = self.state;
        self.state = ElementState::Default;
    }

    /// Set the element state to `ElementState::Clicked`
    fn click(&mut self) {
        self.previous_state = self.state;
        self.state = ElementState::Clicked;
    }

    /// Set the element state to `ElementState::Hovered`
    fn hover(&mut self) {
        // FIXME
        self.previous_state = self.state;
        self.state = ElementState::Hovered;
    }
}

#[derive(Debug)]
pub struct EventManager {
    mouse_pos: Position,
    elements: Vec<Element>,
    callbacks: Vec<EventFn>,
}

impl EventManager {
    pub fn new(cx: EventContext, layout: &dyn Layout) -> Self {
        let elements: Vec<Element> = layout.iter().map(|l| Element::new(l.id())).collect();

        Self {
            elements,
            mouse_pos: Position::default(),
            callbacks: cx.callbacks,
        }
    }

    pub fn elements(&self) -> &[Element] {
        self.elements.as_slice()
    }

    fn process_hover(&mut self, layout: &dyn Layout) {
        let bounds = Bounds::new(layout.position(), layout.size());
        let mouse_pos = self.mouse_pos;
        let element = self
            .elements
            .iter_mut()
            .find(|e| e.id == layout.id())
            .unwrap();

        if bounds.within(&mouse_pos) {
            match element.state {
                ElementState::Default => {
                    element.hover();
                    for callback in &mut self.callbacks {
                        callback.run_hover(layout.id());
                    }
                }
                _ => {}
            }
        } else {
            element.default();
            return;
        }
    }

    fn process_left_click(&mut self, layout: &dyn Layout, state: &winit::event::ElementState) {
        // FIXME shouldn't be unwrapping
        let element = self
            .elements
            .iter_mut()
            .find(|e| e.id == layout.id())
            .unwrap();
        match state {
            &winit::event::ElementState::Pressed => match element.state {
                ElementState::Default => {}
                ElementState::Hovered => {
                    element.click();
                    for callback in &mut self.callbacks {
                        callback.run_click(layout.id());
                    }
                }
                ElementState::Clicked => {}
            },
            &winit::event::ElementState::Released => element.roll_back(),
        }
    }

    fn process_click(
        &mut self,
        layout: &dyn Layout,
        state: &winit::event::ElementState,
        button: &winit::event::MouseButton,
    ) {
        match button {
            winit::event::MouseButton::Left => {
                self.process_left_click(layout, state);
            }
            winit::event::MouseButton::Right => {}
            _ => {}
        }
    }

    fn process_keyboard(&mut self, event: &winit::event::KeyEvent, layout: &dyn Layout) {
        match &event.logical_key {
            winit::keyboard::Key::Character(ch) => {
                for callback in &mut self.callbacks {
                    // impl FROM for characters
                    let _char = ch.chars().into_iter().next().unwrap();
                    callback.run_key(layout.id(), Key::Char(_char));
                }
            }
            _ => {}
        }
    }

    /// Process the incoming `WindowEvent` and dispatch events to [`Widget`]'s
    pub fn process(&mut self, event: &winit::event::WindowEvent, layout: &dyn Layout) {
        // FIXME please handle the panics
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (*position).into();
                for layout in layout.iter() {
                    self.process_hover(layout);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                for layout in layout.iter() {
                    self.process_click(layout, state, button);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.process_keyboard(event, layout);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crystal::{EmptyLayout, Size};
    use winit::{
        dpi::PhysicalPosition,
        event::{DeviceId, ElementState as WinitElementState, MouseButton},
    };

    #[test]
    fn mouse_position_updates() {
        let mut events = EventManager::new(EventContext::new(), &EmptyLayout::default());

        let device_id = unsafe { DeviceId::dummy() };
        let position = PhysicalPosition::new(50.0, 60.0);
        let cursor_event = WindowEvent::CursorMoved {
            device_id,
            position,
        };

        events.process(&cursor_event, &EmptyLayout::default());
        assert_eq!(events.mouse_pos, position.into())
    }

    #[test]
    fn hover_state() {
        let mut layout = EmptyLayout::default();
        layout.size = Size::new(500.0, 500.0);
        let mut events = EventManager::new(EventContext::new(), &layout);

        let device_id = unsafe { DeviceId::dummy() };
        let position = PhysicalPosition::new(92.23, 63.2);

        let cursor_event = WindowEvent::CursorMoved {
            device_id,
            position,
        };
        events.process(&cursor_event, &layout);

        assert_eq!(events.elements[0].state, ElementState::Hovered);

        let position = PhysicalPosition::new(9002.23, 6003.2);
        let cursor_event = WindowEvent::CursorMoved {
            device_id,
            position,
        };
        events.process(&cursor_event, &layout);
        assert_eq!(events.elements[0].state, ElementState::Default);
    }

    #[test]
    fn click_state() {
        let layout = EmptyLayout::default();
        let mut events = EventManager::new(EventContext::new(), &layout);

        let device_id = unsafe { DeviceId::dummy() };
        let click_event = WindowEvent::MouseInput {
            device_id,
            state: WinitElementState::Pressed,
            button: MouseButton::Left,
        };
        events.elements[0].state = ElementState::Hovered;
        events.process(&click_event, &layout);
        assert_eq!(events.elements[0].state, ElementState::Clicked);

        let click_event = WindowEvent::MouseInput {
            device_id,
            state: WinitElementState::Released,
            button: MouseButton::Left,
        };
        events.process(&click_event, &layout);
        assert_eq!(events.elements[0].state, ElementState::Hovered);
    }
}
