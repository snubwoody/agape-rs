use super::{IntoPrimitive, Primitive};
use helium_core::{Color, Position, Size};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Text {
    pub text: String,
    pub font_size: u8,
	/// The line height is a multiple of the font_size, e.g.
	/// a line height of `1.0` with a font size of `16px` is `16px`.
    pub line_height: f32,
    pub color: Color,
    pub position: Position,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
            font_size: 16,
			line_height:1.5,
            ..Default::default()
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

    /// Set the line height
    ///
	/// The line height is a multiple of the font_size, e.g.
	/// a line height of `1.0` with a font size of `16px` is `16px`.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
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
