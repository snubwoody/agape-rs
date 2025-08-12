use crate::resources::{CursorPosition, EventQueue, WindowSize};
use crate::widgets::View;
use crate::{Message, Resources};
use agape_core::Position;
use winit::event::{ElementState, MouseButton, WindowEvent};

pub fn rebuild_widgets(resources: &mut Resources) {
    // let view = resources.get_mut::<Box<dyn View>>().unwrap();
    // view.update(message);
    // let widget = view.view();
    // resources.set(widget);
}

pub fn layout_system(resources: &mut Resources) {
    // FIXME
    let WindowSize(_) = resources.get_owned::<WindowSize>().unwrap();
}

pub fn update_cursor_position(resources: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::CursorMoved { position, .. } = event {
        let cursor = CursorPosition(Position::from(*position));
        resources.set(cursor);
        let message = Message::MouseMoved(Position::from(*position));
        let queue = resources.get_mut::<Vec<Message>>().unwrap();
        queue.push(message);
    }
}

pub fn handle_mouse_button(_resources: &mut Resources, event: &WindowEvent) {
    if let &WindowEvent::MouseInput { state, button, .. } = event {
        if state != ElementState::Pressed || button != MouseButton::Left {
            return;
        }
        dbg!(&state, &button);
    }
}

pub fn handle_key_input(_: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::KeyboardInput { .. } = event {}
}

pub fn intersection_observer(_: &mut Resources) {
    // FIXME
}
