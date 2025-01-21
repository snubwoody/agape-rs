use crate::view::TextView;
use super::Widget;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;

// TODO add editable() method?
/// A [`Widget`] for displaying text onto the screen.
/// 
/// # Example
/// ```
/// use helium::widgets::Text;
/// 
/// Text::new("Hello world");
/// ```
#[derive(Debug,Clone,PartialEq,PartialOrd,Hash)]
pub struct Text {
    id: String,
    pub text: String,
    pub font_size: u8,
    pub color: Color,
}

impl Default for Text {
	fn default() -> Self {
		Self {
            id: nanoid::nanoid!(),
            font_size: 16,
            text: Default::default(),
            color: Color::Hex("#000000"),
        }
	}	
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

	pub fn text(mut self, text: &str) -> Self{
		self.text = text.to_string();
		self
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
    fn id(&self) -> &str {
        &self.id
    }

	fn tick(&mut self,elements:&[crate::events::Element]) {}

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

    fn view(&self) -> Box<dyn crate::view::View> {
        Box::new(
            TextView::new(&self.id, &self.text)
                .font_size(self.font_size)
                .color(self.color),
        )
    }
}
