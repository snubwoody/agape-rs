use agape_renderer::{Renderer, Text};

#[test]
fn render_long_string() {
    let mut renderer = Renderer::new();
    let mut text = String::new();
    for _ in 0..1000 {
        text.push('a')
    }
    let text = Text::new(&text);
    renderer.draw_text(text);
}
