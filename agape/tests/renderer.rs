use agape::Resources;
use agape::style::Border;
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

#[test]
fn draw_border() {
    // TODO failing for small values (1.0)
    let mut pixmap = Pixmap::new(500, 500).unwrap();
    pixmap.fill(tiny_skia::Color::WHITE);

    let background_color = Color::rgb(250, 250, 250);
    let border_color = Color::rgb(100, 20, 105);

    let view = RectView {
        size: Size::unit(500.0),
        border: Some(Border {
            color: border_color.clone(),
            width: 10.0,
        }),
        color: background_color,
        ..Default::default()
    };

    view.render(&mut pixmap, &Resources::new());

    pixmap.save_png("temp_out.png").unwrap();
    for x in 0..500 {
        for y in 0..500 {
            if x >= 5 && y >= 5 && x <= 495 && y <= 495 {
                continue;
            }

            let pixel = pixmap.pixel(x, y).unwrap();
            let mut pixel_color = (pixel.red(), pixel.green(), pixel.blue(), pixel.alpha());
            pixel_color.3 = 100;
            assert_eq!(pixel_color, border_color.inner(), "Failed at ({x},{y})");
        }
    }
}
