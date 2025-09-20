use super::{Text, Widget};
use crate::MessageQueue;
use crate::state::{CharacterInput, NamedKeyInput};
use agape_core::GlobalId;
use agape_layout::{BlockLayout, EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;
use winit::keyboard::NamedKey;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct TextField {
    id: GlobalId,
    pub value: Text,
}

impl TextField {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for TextField {
    fn tick(&mut self, messages: &mut MessageQueue) {
        // TODO check for focus
        if let Some(input) = messages.get::<CharacterInput>() {
            self.value.value.push_str(&input.0);
        }

        if let Some(input) = messages.get::<NamedKeyInput>() {
            match input.0 {
                NamedKey::Backspace => {
                    self.value.value.pop();
                }
                _ => {}
            }
        }
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let child_layout = self.value.layout(renderer);
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
        self.value.render(renderer, layout);
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}
