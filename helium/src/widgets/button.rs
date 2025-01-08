use super::{text::Text, Widget};
use crate::app::events::Event;
use crate::{impl_events, surface::rect::RectSurface, widgets::WidgetBody};
use crystal::{BlockLayout,Layout};
use helium_core::color::Color;

/// A simple button.
pub struct Button {
    text: String,
    color: Color,
    padding: u32,
    corner_radius: u32,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            color: Color::Hex("#615fff"),
            padding: 12,
            corner_radius: 0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

impl Widget for Button {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
		let id = nanoid::nanoid!();
        let mut surface = RectSurface::new(&id);
        surface.color(self.color);
        surface.corner_radius(self.corner_radius);

        let (text_body, text_layout) = Text::new(&self.text).build();

        let body = WidgetBody {
            id: id.clone(),
            surface: Box::new(surface),
            children: vec![Box::new(text_body)],
            ..Default::default()
        };

        let mut layout = BlockLayout::new(text_layout);
        layout.id = id.clone();
        layout.padding = self.padding;

        (body, Box::new(layout))
    }

    fn update(&mut self) {}
}
