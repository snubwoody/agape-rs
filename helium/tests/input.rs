use crystal::Layout;
use helium::event::Event;
use helium::{view::View, widgets::Widget};
use helium_core::{GlobalId, Position};
use winit::event::{ElementState, MouseButton};

struct MockWidget {
    mouse_pos: Position,
    pressed: bool,
}

impl MockWidget {
    pub fn new() -> Self {
        Self {
            pressed: false,
            mouse_pos: Position::default(),
        }
    }
}

impl Widget for MockWidget {
    fn id(&self) -> GlobalId {
        todo!()
    }

    fn view(&self) -> Box<dyn View> {
        todo!()
    }

    fn layout(&self) -> Box<dyn Layout> {
        todo!()
    }

    fn handle_click(&mut self) {
        self.pressed = !self.pressed;
    }

    fn handle_cursor(&mut self, position: Position) {
        self.mouse_pos = position;
    }

    fn handle_text_input(&mut self, text: &str) {
        dbg!(text);
    }
}

#[test]
fn handle_left_click() {
    let mut widget = MockWidget::new();
    assert_eq!(widget.pressed, false);

    let event = Event::MouseInput {
        button: MouseButton::Left,
        state: ElementState::Pressed,
    };
    widget.handle_event(&event);
    assert_eq!(widget.pressed, true);

    let event = Event::MouseInput {
        button: MouseButton::Left,
        state: ElementState::Released,
    };
    widget.handle_event(&event);
    assert_eq!(widget.pressed, true);
}

#[test]
fn handle_cursor() {
    let mut widget = MockWidget::new();
    let event = Event::CursorMoved(Position::unit(100.0));
    widget.handle_event(&event);

    assert_eq!(widget.mouse_pos, Position::unit(100.0));
}
