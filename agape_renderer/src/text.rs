use agape_core::{Color, IntoColor, Position, Rgba, Size};
use cosmic_text::fontdb::Query;
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Style, SwashCache, Weight};
use image::RgbaImage;
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

// TODO: add line height
#[derive(Clone, PartialEq, Debug)]
pub struct Text<'a> {
    pub content: String,
    pub font_size: f32,
    /// The line height is a multiple of the font size.
    pub line_height: f32,
    pub position: Position,
    pub font: FontQuery<'a>,
    pub color: Color,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<'a> Text<'a> {
    /// Create a new [`Text`] primitive, with a default font
    /// size of 16px.
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            font_size: 16.0,
            line_height: 1.25,
            font: FontQuery::default(),
            position: Position::default(),
            color: Color::BLACK,
        }
    }

    /// Set the font size;
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the text color.
    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    /// Set the line height.
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }

    /// Set the font weight
    pub fn weight(mut self, weight: Weight) -> Self {
        self.font.weight = weight;
        self
    }

    /// Add a font family. Font families will be queried in the order
    /// they are pushed and the first match will be used.
    pub fn add_family(mut self, family: Family<'a>) -> Self {
        self.font.add_family(family);
        self
    }

    /// Get the font metric
    fn metrics(&self) -> Metrics {
        Metrics::relative(self.font_size, self.line_height)
    }

    pub fn size(&self, font_system: &mut FontSystem) -> Size {
        // FIXME: add line height
        let metrics = self.metrics();
        let attrs = self.attrs(font_system);
        let mut buffer = Buffer::new(font_system, metrics);
        let mut buffer = buffer.borrow_with(font_system);

        buffer.set_text(&self.content, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);

        let mut width = 0.0;
        let mut height = 0.0;

        for run in buffer.layout_runs() {
            width += run.line_w;
            height += run.line_height;
        }

        Size::new(width, height)
    }

    fn query_font(&self, font_system: &mut FontSystem) -> Option<Family<'a>> {
        let query: Query = Query {
            families: self.font.families.as_ref(),
            weight: self.font.weight,
            style: self.font.style,
            ..Default::default()
        };
        let db = font_system.db();
        let font_id = db.query(&query);
        let family_name = if let Some(id) = font_id
            && let Some(face) = db.face(id)
        {
            let name = &face.families[0].0;
            Some(name)
        } else {
            None
        };

        if let Some(name) = family_name {
            for family in &self.font.families {
                if let Family::Name(_name) = family
                    && name == _name
                {
                    return Some(*family);
                }
            }
        }

        None
    }

    fn attrs(&self, font_system: &mut FontSystem) -> Attrs<'_> {
        let family = self.query_font(font_system).unwrap_or(Family::SansSerif);
        Attrs::new()
            .family(family)
            .style(self.font.style)
            .weight(self.font.weight)
    }

    pub fn draw_text(
        &self,
        pixmap: &mut Pixmap,
        font_system: &mut FontSystem,
        cache: &mut SwashCache,
    ) {
        // TODO: cache text that has the same content and font size
        // TODO: check families that don't exist
        let metrics = self.metrics();
        let attrs = self.attrs(font_system);
        let size = self.size(font_system);

        let mut buffer = Buffer::new(font_system, metrics);
        let mut buffer = buffer.borrow_with(font_system);
        buffer.set_text(&self.content, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);

        // TODO: add clippy lint for conversion
        let (r, g, b, a) = self.color.inner();
        let text_color = cosmic_text::Color::rgba(r, g, b, a);
        let width = size.width.ceil() as u32;
        let height = size.height.ceil() as u32;

        let mut image = RgbaImage::new(width, height);
        let size = IntSize::from_wh(image.width(), image.height()).unwrap();

        buffer.draw(cache, text_color, |x, y, _, _, color| {
            let [r, g, b, a] = color.as_rgba();
            let x = x as u32;
            let y = y as u32;
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, image::Rgba([r, g, b, a]));
            }
        });

        let glyph_pixmap = Pixmap::from_vec(image.to_vec(), size).unwrap();
        let Position { x, y } = self.position;
        pixmap.draw_pixmap(
            x as i32,
            y as i32,
            glyph_pixmap.as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontQuery<'a> {
    /// A prioritized list of font family names or generic family names.
    ///
    /// [font-family](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#propdef-font-family) in CSS.
    pub families: Vec<Family<'a>>,
    /// Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
    ///
    /// [font-weight](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-weight-prop) in CSS.
    pub weight: Weight,
    /// Allows italic or oblique faces to be selected.
    ///
    /// [font-style](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-style-prop) in CSS.
    pub style: Style,
}

impl<'a> FontQuery<'a> {
    /// Create a new font query.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_family(&mut self, family: Family<'a>) {
        self.families.push(family);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn font_metrics() {
        let text = Text::new("Hello world").font_size(10.0).line_height(1.2);

        let metrics = text.metrics();
        assert_eq!(metrics.font_size, 10.0);
        assert_eq!(metrics.line_height, 12.0);
    }

    #[test]
    fn get_matched_font() {
        let mut font_system = FontSystem::new();
        let text = Text::new("Hello world").add_family(Family::Name("Segoe UI"));
        let family = text.query_font(&mut font_system);
        assert_eq!(family.unwrap(), Family::Name("Segoe UI"));
    }
}
