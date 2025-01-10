use super::Widget;
use crate::surface::Primitive;
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
	fn layout(&self) -> Box<dyn Layout> {
		// FIXME hopefully a temp fix because i don't know how to calculate the size before hand
		let text_renderer = text_to_png::TextRenderer::default();

        // Render the text as a png to get the size
        let text_image = text_renderer
            .render_text_to_png_data(self.text.clone(), self.font_size, "#000000")
            .unwrap();

		

		let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(text_image.size.width as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(text_image.size.height as f32);
        layout.id = self.id.clone();

		Box::new(layout)
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
