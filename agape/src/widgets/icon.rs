use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::Widget;
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use usvg::Tree;

/// Displays an icon onto the screen.
#[derive(Debug)]
pub struct Icon {
    id: GlobalId,
    data: Arc<Tree>,
    style: BoxStyle,
}

impl Icon {
    pub fn asset(path: impl AsRef<Path>) {}

    impl_style! {}
}

impl Widget for Icon {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

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
        let mut svg = agape_renderer::Svg::new(self.data.clone());
        svg.size = size;
        svg.position = position;
        renderer.draw_svg(svg);
    }
}
