use super::{LayoutDescription, RenderBox, RenderObject, Widget};
use agape_core::Color;
use agape_core::{GlobalId, Position, Size};

#[derive(Clone, PartialEq, Debug)]
pub struct Text {
    id: GlobalId,
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
        RenderBox {
            id: self.id,
            layout_desc: LayoutDescription::default(),
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
