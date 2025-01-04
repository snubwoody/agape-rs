use super::{text::Text, Widget};
use crate::app::events::Event;
use crate::{
    impl_events,
    surface::rect::RectSurface,
    widgets::WidgetBody,
};
use crystal::{BlockLayout, BoxSizing, Layout};
use helium_core::color::Color;
use nanoid::nanoid;
use winit::keyboard;

/// A simple button.
pub struct Button {
    id: String,
    text: String,
    color: Color,
    padding: u32,
    corner_radius: u32,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            id: nanoid!(),
            text: text.into(),
            color: Color::Hex("#615fff"),
            padding: 12,
            corner_radius: 0,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
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

    impl_events!();
}

impl Widget for Button {
    fn build(&self) -> (WidgetBody,Box<dyn Layout>) {
        let mut surface = RectSurface::default();
		surface.color = self.color.clone();
        surface.corner_radius(self.corner_radius);
		
        let (text_body,text_layout) = Text::new(&self.text).build();
		
		
		let body = WidgetBody {
			id: self.id.clone(),
            surface: Box::new(surface),
            children: vec![Box::new(text_body)],
            ..Default::default()
        };
		
		let mut layout = BlockLayout::new(text_layout);
		layout.id = body.id.clone();
		layout.padding = self.padding;

		(body,Box::new(layout))
    }

	fn update(&mut self) {
		
	}
}
