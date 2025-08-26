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

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();

        let mut rect = agape_renderer::rect::Rect::new()
            .size(size.width, size.height)
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius)
            .color(self.style.background_color.clone());

        rect.border = self.style.border.clone();
        renderer.draw_rect(rect);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use agape_core::{Color, Size};
    use agape_layout::solve_layout;

    #[test]
    fn background_color() {
        let rect = Rect::new()
            .fixed(100.0, 100.0)
            .background_color(Color::rgb(53, 102, 145));

        let mut pixmap = Pixmap::new(100, 100).unwrap();
        let mut renderer = Renderer::new();
        let mut layout = rect.layout(&mut renderer);
        solve_layout(layout.as_mut(), Size::default());
        rect.render(&mut renderer, layout.as_ref());

        for pixel in pixmap.pixels() {
            assert_eq!(pixel.red(), 53);
            assert_eq!(pixel.green(), 102);
            assert_eq!(pixel.blue(), 145);
        }
    }
}
