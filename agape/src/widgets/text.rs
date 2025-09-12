use super::Widget;
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;

/// Draw text onto the screen.
///
/// Emojis are currently unsupported.
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

    fn set_id(&mut self, id: GlobalId) {
        self.id = id
    }
    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let size = renderer.text_size(&self.value, self.font_size as f32);
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: IntrinsicSize::from(size),
            ..Default::default()
        };
        Box::new(layout)
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {}

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let position = layout.position();
        let mut text = agape_renderer::Text::new(self.value.as_str());
        text.font_size = self.font_size as f32;
        text.position = position;
        renderer.draw_text(text)
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}
