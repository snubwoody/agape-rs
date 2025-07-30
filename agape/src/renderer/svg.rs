use agape_core::{Position, Size};
use tiny_skia::{Pixmap, Transform};
use usvg::Tree;

// TODO: add support for custom colors
pub fn draw_svg(pixmap: &mut Pixmap, tree: &Tree, position: Position, size: Size) {
    let svg_width = tree.size().width();
    let svg_height = tree.size().height();
    let scale_x = size.width / svg_width;
    let scale_y = size.height / svg_height;
    let transform = Transform::from_translate(position.x, position.y).post_scale(scale_x, scale_y);

    resvg::render(&tree, transform, &mut pixmap.as_mut());
}

#[cfg(test)]
mod test {
    use crate::renderer::svg::draw_svg;
    use agape_core::{Position, Size};
    use tiny_skia::Pixmap;

    #[test]
    fn render_svg() {
        let mut pixmap = Pixmap::new(200, 200).unwrap();
        let data = include_bytes!("../../icons/feather-icons/airplay.svg");
        let options = usvg::Options::default();
        let tree = usvg::Tree::from_data(data, &options).unwrap();
        draw_svg(&mut pixmap, &tree, Position::default(), Size::unit(150.0));
    }
}
