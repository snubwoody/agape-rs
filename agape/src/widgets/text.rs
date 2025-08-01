use super::{LayoutDescription, RenderBox, RenderObject, Widget};
use crate::renderer::text_size;
use agape_core::Color;
use agape_core::{GlobalId, Position, Size};

/// Draw text onto the screen.
///
/// Emojis are currently unsupported.
#[derive(Clone, PartialEq, Debug)]
pub struct Text {
    id: GlobalId,
    // TODO: rename to inner or content
    pub text: String,
    pub font_size: u8,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            id: GlobalId::new(),
            text: String::new(),
            font_size: 16,
        }
    }
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            ..Default::default()
        }
    }

    /// Set the font size of the `Text` widget.
    ///
    /// ```
    /// use agape::widgets::Text;
    /// let message = Text::new("Foo").font_size(12);
    ///
    /// assert_eq!(message.font_size,12);
    /// ```
    pub fn font_size(mut self, font_size: u8) -> Self {
        self.font_size = font_size;
        self
    }
}

impl Widget for Text {
    fn build(&self) -> RenderBox {
        let text_size = text_size(&self.text, self.font_size as f32);

        RenderBox {
            id: self.id,
            layout_desc: LayoutDescription {
                intrinsic_size: text_size.into(),
                ..Default::default()
            },
            children: Vec::new(),
            position: Position::default(),
            size: Size::default(),
            render_object: RenderObject::Text {
                content: self.text.clone(),
                font_size: self.font_size,
                color: Color::BLACK,
            },
        }
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::FONT;
    use crate::renderer::{init_font, text_size};
    use agape_layout::IntrinsicSize;

    #[test]
    fn correct_text_size() {
        FONT.set(init_font()).unwrap();
        let text = Text::new("Hello world").font_size(24);
        let text_size = text_size("Hello world", 24.0);
        let rb = text.build();

        assert_eq!(
            rb.layout_desc.intrinsic_size,
            IntrinsicSize::from(text_size)
        );
    }
}
