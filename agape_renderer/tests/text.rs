use agape_core::Position;
use agape_renderer::Renderer;
use tiny_skia::Pixmap;

#[test]
fn render_long_string() {
    let mut renderer = Renderer::new();
    let mut pixmap = Pixmap::new(512, 512).unwrap();
    let mut text = String::new();
    for _ in 0..1000 {
        text.push('a')
    }
    renderer.draw_text(&mut pixmap, &text, 16.0, Position::default());
}
