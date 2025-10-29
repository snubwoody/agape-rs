use super::Widget;
use agape_core::{Color, GlobalId, IntoColor, Rgba};
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::{Family, Renderer, Style, Weight};

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
    pub color: Color,
    pub weight: Weight,
    pub line_height: f32,
    pub style: Style,
    pub families: Vec<String>,
}

// TODO: add TextStyle
impl Default for Text {
    fn default() -> Text {
        Text {
            id: GlobalId::new(),
            value: String::new(),
            font_size: 16,
            line_height: 1.25,
            color: Color::BLACK,
            families: Vec::new(),
            style: Style::default(),
            weight: Weight::default(),
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

    pub fn family(mut self, family: &str) -> Self {
        self.families.push(family.to_owned());
        self
    }

    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
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

    fn as_text(&self) -> agape_renderer::Text<'_> {
        let mut text = agape_renderer::Text::new(self.value.as_str())
            .font_size(self.font_size as f32)
            .line_height(self.line_height)
            .color(self.color.clone())
            .weight(self.weight)
            .style(self.style);

        for family in &self.families {
            text = text.add_family(Family::Name(family.as_str()));
        }

        text
    }
}

impl Widget for Text {
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let text = self.as_text();
        let size = renderer.text_size(text);
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: IntrinsicSize::from(size),
            ..Default::default()
        };
        Box::new(layout)
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        if self.value.is_empty() {
            return;
        }

        let layout = layout.get(self.id).unwrap();
        let position = layout.position();
        let mut text = self.as_text();
        text.position = position;
        renderer.draw_text(text)
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text::new(&value)
    }
}

impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Text::new(value.as_str())
    }
}
