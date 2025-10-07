use super::Widget;
use crate::element::{Element, ElementKind};
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;

/// Draw text onto the screen. Emojis are fully supported.
///
/// # Example
///
/// ```
/// use agape::widgets::Text;
///
/// let text = Text::new("🦀 Hi!")
///     .font_size(20);
///
/// assert_eq!(text.font_size,20);
/// assert_eq!(text.value,"🦀 Hi!");
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Text {
    id: GlobalId,
    pub value: String,
    pub font_size: u8,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            id: GlobalId::new(),
            value: String::new(),
            font_size: 16,
        }
    }
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            value: text.to_owned(),
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
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn build(&self) -> Element {
        let kind = ElementKind::Text {
            value: self.value.clone(),
            font_size: self.font_size as f32,
        };

        Element {
            id: self.id,
            kind,
            on_click: None,
            label: String::from("Text"),
            children: Vec::new(),
        }
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}
