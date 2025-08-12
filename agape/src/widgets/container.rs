use super::{LayoutDescription, LayoutType, RenderBox, RenderObject, Widget};
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use agape_renderer::Renderer;
use tiny_skia::Pixmap;

/// A widget that wraps another widget.
pub struct Container<W> {
    id: GlobalId,
    child: W,
    style: BoxStyle,
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

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        f(&self.child);
        self.child.traverse(f);
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
        self.child.traverse_mut(f);
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let child = self.child.layout(renderer);
        let mut layout = BlockLayout::new(child);
        layout.id = self.id;
        layout.intrinsic_size = self.style.intrinsic_size;
        Box::new(layout)
    }

    fn render(&self, pixmap: &mut Pixmap, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        renderer.draw_rect(
            pixmap,
            &self.style.background_color.clone(),
            size,
            position,
            self.style.border.clone(),
        );
        self.child.render(pixmap, renderer, layout);
    }

    fn build(&self, renderer: &mut Renderer) -> RenderBox {
        let render_object = RenderObject::Rect {
            border: self.style.border.clone(),
            color: self.style.background_color.clone(),
        };

        // TODO: add padding and alignment
        let layout = LayoutDescription {
            layout_type: LayoutType::BlockLayout,
            intrinsic_size: self.style.intrinsic_size,
            ..Default::default()
        };

        let child = self.child.build(renderer);
        let mut render_box = RenderBox::new(self.id, layout, render_object);
        render_box.children.push(child);
        render_box
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

        let mut pixmap = Pixmap::new(100, 100).unwrap();
        let mut renderer = Renderer::new();
        let mut layout = container.layout(&mut renderer);
        solve_layout(layout.as_mut(), Size::default());
        container.render(&mut pixmap, &mut renderer, layout.as_ref());

        for pixel in pixmap.pixels() {
            assert_eq!(pixel.red(), 53);
            assert_eq!(pixel.green(), 102);
            assert_eq!(pixel.blue(), 145);
        }
    }
}
