use super::{LayoutDescription, LayoutType, RenderBox, RenderObject, Widget};
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::GlobalId;
use agape_renderer::Renderer;

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
