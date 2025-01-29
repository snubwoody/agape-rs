use super::{IntoPrimitive, Primitive};
use helium_core::{Color, Position, Size};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Text {
    pub text: String,
    pub font_size: u8,
    pub color: Color,
    pub position: Position,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
            font_size: 16,
            ..Default::default()
        }
    }

    pub fn size(&self) -> Size {
        let text_renderer = text_to_png::TextRenderer::default();

        // Render the text as a png
        let text_image = text_renderer
            .render_text_to_png_data(
                self.text.clone(),
                self.font_size,
                self.color.into_hex_string().as_str(),
            )
            .unwrap();

        Size {
            width: text_image.size.width as f32,
            height: text_image.size.height as f32,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }

    /// Set the font size
    ///
    /// # Panics
    /// Font size of 0 panics
    pub fn font_size(mut self, font_size: u8) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl IntoPrimitive for Text {
    fn into_primitive(self) -> Primitive {
        Primitive::Text(self)
    }
}
