use super::Widget;
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::{GlobalId, Position, Size};
use agape_layout::{EmptyLayout, Layout};
use agape_renderer::Renderer;
use tiny_skia::Pixmap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Rect {
    id: GlobalId,
    size: Size,
    position: Position,
    style: BoxStyle,
}

impl Rect {
    pub fn new() -> Self {
        Self::default()
    }

    impl_style!();
}

impl Widget for Rect {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout> {
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            ..Default::default()
        };
        Box::new(layout)
    }

    fn render(&self, pixmap: &mut Pixmap, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();

        let mut rect = agape_renderer::rect::Rect::new()
            .size(size.width, size.height)
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius);

        rect.border = self.style.border.clone();
        renderer.draw_rect(pixmap, rect);
    }
}
