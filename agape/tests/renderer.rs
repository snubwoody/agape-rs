use agape::Resources;
use agape::view::{RectView, View};
use agape_core::{Color, Size};
use tiny_skia::Pixmap;

#[test]
fn set_correct_pixel_color() {
    let mut pixmap = Pixmap::new(500, 500).unwrap();
    pixmap.fill(tiny_skia::Color::WHITE);
    let color = Color::rgb(25, 120, 97);
    let mut rect_view = RectView {
        color,
        ..Default::default()
    };
    rect_view.size = Size::unit(500.0);
    rect_view.render(&mut pixmap, &Resources::new());

    for pixel in pixmap.pixels() {
        let r = pixel.red();
        let g = pixel.green();
        let b = pixel.blue();
        assert_eq!(r, 25);
        assert_eq!(g, 120);
        assert_eq!(b, 97);

        if !pixel.is_opaque() {
            panic!("Incorrect pixel alpha");
        }
    }
}
