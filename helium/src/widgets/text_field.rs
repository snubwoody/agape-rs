use super::{Text, Widget};
use crate::{
    colors::tailwind_colors::{BLUE100, GRAY100, GREEN100, RED100},
    events::Element,
};
use crystal::BlockLayout;
use helium_core::color::{Color, INDIGO};
use helium_renderer::Rect;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Contains editable text
pub struct TextField {
    id: String,
    text: Text,
    background_color: Color,
}

impl TextField {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(),
            text: Text::new("Placeholder"),
            background_color: Color::default(),
        }
    }

    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        self
    }

    fn on_input(&mut self) {}
}

impl Widget for TextField {
    fn id(&self) -> &str {
        &self.id
    }

    fn tick(&mut self, elements: &[Element]) {
    }

	fn process_key(&mut self,key:&winit::keyboard::Key) {
		dbg!(&key);
	}

    fn layout(&self) -> Box<dyn crystal::Layout> {
        let child_layout = self.text.layout();
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id.clone();
        Box::new(layout)
    }

	fn draw(&self,layout:&dyn crystal::Layout,renderer:&mut helium_renderer::Renderer) {
		renderer.draw([ // TODO impl From<Layout>
			Rect::new(layout.size().width, layout.size().height)
				.position(layout.position().x, layout.position().y)
				.color(self.background_color)
		]);
	}
}
