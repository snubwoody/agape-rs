use agape_core::{Position, Size};
use std::sync::Arc;
use tiny_skia::{Pixmap, Transform};
use usvg::Tree;

pub struct Svg {
    tree: Arc<Tree>,
    pub size: Size,
    pub position: Position,
}

impl Svg {
    pub fn new(tree: Arc<Tree>) -> Self {
        Self {
            size: Size::from(tree.size()),
            position: Position::default(),
            tree,
        }
    }

    pub fn draw(&self, pixmap: &mut Pixmap) {
        let svg_width = self.tree.size().width();
        let svg_height = self.tree.size().height();
        let scale_x = self.size.width / svg_width;
        let scale_y = self.size.height / svg_height;
        let transform = Transform::from_scale(scale_x, scale_y)
            .post_translate(self.position.x, self.position.y);

        resvg::render(&self.tree, transform, &mut pixmap.as_mut());
    }
}
