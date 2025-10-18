use agape_renderer::{Renderer, Text};
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
