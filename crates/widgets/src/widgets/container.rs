use super::Widget;
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

/// A widget that wraps another widget.
#[derive(Clone)]
pub struct Container<W> {
    id: GlobalId,
    pub child: W,
    pub style: BoxStyle,
}

impl<W> Container<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: GlobalId::new(),
            style: BoxStyle::new(),
            child,
        }
    }

    impl_style!();
}

impl<W: Widget> Widget for Container<W> {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
        self.child.traverse(f);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let child = self.child.layout(renderer);
        let mut layout = BlockLayout::new(child);
        layout.id = self.id;
        layout.padding = self.style.padding;
        layout.intrinsic_size = self.style.intrinsic_size;
        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        let mut rect = Rect::new()
            .size(size.width, size.height)
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius)
            .color(self.style.background_color.clone());

        rect.border = self.style.border.clone();
        renderer.draw_rect(rect);
        self.child.render(renderer, layout);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use agape_core::{Color, Size};
    use agape_layout::solve_layout;

    #[test]
    fn render_child() {
        let container = Container::new(
            Rect::new()
                .fixed(100.0, 100.0)
                .background_color(Color::rgb(53, 102, 145)),
        )
        .fixed(100.0, 100.0);

        let mut renderer = Renderer::new();
        renderer.resize(100, 100);
        let mut layout = container.layout(&mut renderer);
        solve_layout(layout.as_mut(), Size::default());
        container.render(&mut renderer, layout.as_ref());

        for pixel in renderer.pixmap().pixels() {
            assert_eq!(pixel.red(), 53);
            assert_eq!(pixel.green(), 102);
            assert_eq!(pixel.blue(), 145);
        }
    }
}
