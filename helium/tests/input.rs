use crystal::Layout;
use helium::event::Event;
use helium::{view::View, widgets::Widget};
use helium_core::GlobalId;
use winit::event::{ElementState, MouseButton};

struct MockWidget {
    pressed: bool,
}

impl MockWidget {
    pub fn new() -> Self {
        Self { pressed: false }
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
