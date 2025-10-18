use agape_core::Color;
use agape_renderer::{FontQuery, Renderer, Text};
use cosmic_text::fontdb::Query;
use cosmic_text::{Family, Weight};

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

#[test]
fn render_output() {
    // TODO: use platform specific fallback
    let mut renderer = Renderer::new();
    renderer.load_fonts_dir("assets/Work Sans/static");
    let text = Text::new("Hello world")
        .add_family(Family::Name("Work Sans"))
        .font_size(50.0)
        .weight(Weight(400));
    renderer.pixmap_mut().fill(tiny_skia::Color::WHITE);
    renderer.draw_text(text);
    renderer.pixmap_mut().save_png("temp-output.png").unwrap();
}

#[test]
fn load_font_file() -> Result<(), std::io::Error> {
    let mut renderer = Renderer::new();

    let query = Query {
        families: &[Family::Name("Work Sans")],
        ..Default::default()
    };
    let font_face = renderer.db().query(&query);
    assert!(font_face.is_none());
    renderer.load_font_file("assets/Work Sans/static/WorkSans-Regular.ttf")?;
    let font_face = renderer.db().query(&query);
    assert!(font_face.is_some());
    Ok(())
}

#[test]
fn load_font_dir() -> Result<(), std::io::Error> {
    let mut renderer = Renderer::new();
    let len = renderer.db().len();
    renderer.load_fonts_dir("assets/Work Sans/static");
    assert_eq!(renderer.db().len(), len + 18);
    Ok(())
}

#[test]
fn load_variable_font_file() -> Result<(), std::io::Error> {
    let mut renderer = Renderer::new();
    let query = Query {
        families: &[Family::Name("Work Sans")],
        weight: Weight::NORMAL,
        ..Default::default()
    };
    let query_bold = Query {
        families: &[Family::Name("Work Sans")],
        weight: Weight::BOLD,
        ..Default::default()
    };
    let font_face = renderer.db().query(&query);
    assert!(font_face.is_none());

    renderer.load_font_file("assets/Work Sans/WorkSans-VariableFont.ttf")?;
    let normal_face = renderer.db().query(&query).unwrap();
    let bold_face = renderer.db().query(&query_bold).unwrap();
    assert_eq!(normal_face, bold_face);
    Ok(())
}
