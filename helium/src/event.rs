use helium_core::Position;
use winit::event::{ElementState, MouseButton, WindowEvent};

pub enum Event {
    /// The cursor has moved in the window.
    ///
    /// Contains the x and y coords relative to the top left corner
    /// of the window.
    CursorMoved(Position),
    /// The cursor has entered the window.
    CursorEntered,
    /// The cursor has left the window.
    CursorLeft,
    /// A mouse button press.
    MouseInput {
        state: ElementState,
        button: MouseButton,
    },
}

impl Event {
    pub fn from_window_event(event: &WindowEvent) -> Option<Self> {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                Some(Event::CursorMoved((*position).into()))
            }
            WindowEvent::CursorEntered { .. } => Some(Event::CursorEntered),
            WindowEvent::CursorLeft { .. } => Some(Event::CursorLeft),
            WindowEvent::MouseInput { state, button, .. } => Some(Event::MouseInput {
                state: *state,
                button: *button,
            }),
            _ => None,
        }
    }
}
