use winit::event::{ElementState, MouseButton};
use helium_core::Position;

pub enum Event{
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
    MouseInput{
        state: ElementState,
        button: MouseButton,
    },
    /// A touch event.
    Touch{
        
    }
}

