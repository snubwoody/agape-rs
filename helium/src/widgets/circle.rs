use super::{Widget, WidgetBody};
use crate::surface::circle::CircleSurface;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;

#[derive(Debug, Clone)]
pub struct Circle {
    diameter: u32,
    color: Color,
}

impl Circle {
    pub fn new(diameter: u32, color: Color) -> Self {
        Self { diameter, color }
    }
}

impl Widget for Circle {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
		let id = nanoid::nanoid!();
        let mut surface = CircleSurface::new(&id,self.diameter);
		surface.color(self.color);
        
		let body = WidgetBody {
            surface: Box::new(surface),
            ..Default::default()
        };

        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
        layout.id = id.clone();

        (body, Box::new(layout))
    }

    fn update(&mut self) {}
}
