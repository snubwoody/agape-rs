use super::Widget;
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use tiny_skia::Pixmap;

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
    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let size = renderer.text_size(&self.value, self.font_size as f32);
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: IntrinsicSize::from(size),
            ..Default::default()
        };
        Box::new(layout)
    }

    fn render(&self, pixmap: &mut Pixmap, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let position = layout.position();
        renderer.draw_text(pixmap, &self.value, self.font_size as f32, position)
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}
