use super::Widget;
use crate::view::{TextView, View};
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};

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
    fn view(&self) -> Box<dyn View> {
        let mut view = TextView::new(&self.text);
        view.set_id(self.id);
        view.font_size = self.font_size;
        Box::new(view)
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::FONT;
    use crate::view::init_font;

    #[test]
    fn view_has_correct_id() {
        let text = Text::new("Hello");
        let view = text.view();
        assert_eq!(text.id, view.id());
    }
}
