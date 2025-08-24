use agape_core::{Color, Position, Size};
use agape_renderer::Renderer;
use tiny_skia::Pixmap;

#[test]
fn rounded_corners() {
    let mut renderer = Renderer::new();
    let mut pixmap = Pixmap::new(1000, 1000).unwrap();
    pixmap.fill(tiny_skia::Color::WHITE);
    renderer.draw_rect(
        &mut pixmap,
        &Color::BLACK,
        Size::unit(200.0),
        Position::unit(200.0),
        20,
        None,
    );
}
