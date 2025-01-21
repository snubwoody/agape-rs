use super::{Text, Widget};
use crate::{
    colors::tailwind_colors::{BLUE100, GRAY100, GREEN100, RED100},
    events::Element,
    view::{RectView, TextView},
};
use crystal::{BlockLayout, EmptyLayout};
use helium_core::color::{Color, INDIGO};
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
        println!("Hi");

        let colors = [BLUE100, RED100, GRAY100, GREEN100, INDIGO];

        if let Some(&random_color) = colors.choose(&mut thread_rng()) {
            self.background_color = random_color;
        }

        self.text.text.push('H');
    }

    fn layout(&self) -> Box<dyn crystal::Layout> {
        let child_layout = self.text.layout();
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id.clone();
        Box::new(layout)
    }

    fn view(&self) -> Box<dyn crate::view::View> {
        Box::new(RectView::new(&self.id).color(self.background_color))
    }
}
