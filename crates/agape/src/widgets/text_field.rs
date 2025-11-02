use super::{Container, Text, Widget};
use crate::MessageQueue;
use crate::state::{CharacterInput, NamedKeyInput, StateMap};
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;
use tracing::trace;
use winit::keyboard::NamedKey;

type Callback = Option<Box<dyn FnMut(&str, &mut MessageQueue)>>;

struct TextFieldState {
    value: String,
    focused: bool,
}

pub struct TextField {
    id: GlobalId,
    pub child: Container<Text>,
    focused: bool,
    on_change: Callback,
}

impl TextField {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run a callback when the text field value is updated.
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: FnMut(&str, &mut MessageQueue) + 'static,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl Default for TextField {
    fn default() -> Self {
        let child = Container::new(Text::new(""))
            .padding_all(12.0)
            .border_width(1.0)
            .border_color(0)
            .corner_radius(12);

        Self {
            id: GlobalId::new(),
            child,
            focused: false,
            on_change: None,
        }
    }
}

impl Widget for TextField {
    fn click(&mut self, _: &mut MessageQueue) {
        self.focused = !self.focused;
        // TODO: add on submit when enter key is pressed
        trace!("Input ({}) focus state change: {}", self.id, self.focused);
    }

    fn state(&self, index: usize, state_map: &mut StateMap) {
        let state = TextFieldState {
            focused: self.focused,
            value: self.child.child.value.clone(),
        };
        state_map.insert(index, state);
    }

    fn get_state(&mut self, index: usize, state_map: &mut StateMap) {
        if let Some(value) = state_map.get::<TextFieldState>(index) {
            self.child.child.value = value.value.clone();
            self.focused = value.focused;
        }
    }

    fn tick(&mut self, messages: &mut MessageQueue) {
        if !self.focused {
            return;
        }

        // TODO check for focus
        if let Some(input) = messages.get::<CharacterInput>() {
            self.child.child.value.push_str(&input.0);
            if let Some(f) = self.on_change.as_mut() {
                f(self.child.child.value.as_str(), messages);
            }
        }

        if let Some(input) = messages.get::<NamedKeyInput>() {
            match input.0 {
                NamedKey::Backspace => {
                    self.child.child.value.pop();
                    if let Some(f) = self.on_change.as_mut() {
                        f(self.child.child.value.as_str(), messages);
                    }
                }
                NamedKey::Space => {
                    self.child.child.value.push(' ');
                    if let Some(f) = self.on_change.as_mut() {
                        f(self.child.child.value.as_str(), messages);
                    }
                }
                _ => {}
            }
        }
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let child_layout = self.child.layout(renderer);
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id;
        Box::new(layout)
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let position = layout.position();
        // TODO use Into<Position>
        let rect = Rect::new()
            .position(position.x, position.y)
            .size(layout.size().width, layout.size().height);
        renderer.draw_rect(rect);
        self.child.render(renderer, layout);
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}
