use super::{Widget, WidgetBody};
use crate::surface::{text::TextSurface, Primitive, Surface};
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;

// TODO add background, foreground color,padding and border radius
// TODO probably crate a rich text then make text a tuple struct or a function
pub struct Text {
    id: String,
    text: String,
    font_size: u8,
    color: Color,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            id: nanoid::nanoid!(),
            text: text.into(),
            font_size: 16,
            color: Color::Hex("#000000"),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the font size
    pub fn font_size(mut self, size: u8) -> Self {
        self.font_size = size;
        self
    }
}

impl Widget for Text {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
        // Create the text surface to be rendered
        let textsurface =
            TextSurface::new(&self.id, self.text.as_str(), self.font_size, &self.color);

        let size = textsurface.get_size();
        let surface = Box::new(textsurface);

        let body = WidgetBody {
            id: self.id.clone(),
            surface,
            ..Default::default()
        };

        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(size.width);
        layout.intrinsic_size.height = BoxSizing::Fixed(size.height);
        layout.id = self.id.clone();

        (body, Box::new(layout))
    }

    fn surface(&self) -> Vec<Box<dyn Surface>> {
        vec![Box::new(TextSurface::new(
            &self.id,
            self.text.as_str(),
            self.font_size,
            &self.color,
        ))]
    }

	fn primitive(&self) -> Primitive {
		Primitive::Text { 
			id: self.id.clone(), 
			text: self.text.clone(), 
			font_size: self.font_size, 
			color: self.color 
		}
	}
}
