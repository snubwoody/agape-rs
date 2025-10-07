use crate::style::BoxStyle;
use crate::widgets::Widget;
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, HorizontalLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

// TODO: State arena
pub trait Element1 {
    /// Get the `id` of the [`Widget`].
    fn id(&self) -> GlobalId;

    /// Construct a [`Layout`] to solve layout for the whole
    /// widget tree.
    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;

    /// Draw the widget to the screen.
    fn render(&self, _: &mut Renderer, _: &dyn Layout);
}

pub struct Element {
    pub id: GlobalId,
    pub kind: ElementKind,
}

impl Element {
    pub fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        match &self.kind {
            ElementKind::Text { font_size, value } => {
                let size = renderer.text_size(&value, *font_size);
                let layout = EmptyLayout {
                    id: self.id,
                    intrinsic_size: IntrinsicSize::from(size),
                    ..Default::default()
                };
                Box::new(layout)
            }
            ElementKind::Rect { style } => {
                let layout = EmptyLayout {
                    id: self.id,
                    intrinsic_size: style.intrinsic_size,
                    ..Default::default()
                };
                Box::new(layout)
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        match &self.kind {
            ElementKind::Text { font_size, value } => {
                if value.trim().is_empty() {
                    return;
                }

                let layout = layout.get(self.id).unwrap();
                let position = layout.position();
                let mut text = agape_renderer::Text::new(value.as_str());
                text.font_size = *font_size;
                text.position = position;
                renderer.draw_text(text)
            }
            ElementKind::Rect { style } => {
                let layout = layout.get(self.id).unwrap();
                let size = layout.size();
                let position = layout.position();

                let mut rect = Rect::new()
                    .size(size.width, size.height)
                    .position(position.x, position.y)
                    .corner_radius(style.corner_radius)
                    .color(style.background_color.clone());

                rect.border = style.border.clone();
                renderer.draw_rect(rect);
            }
        }
    }
}

pub enum ElementKind {
    Text { font_size: f32, value: String },
    Rect { style: BoxStyle },
}
