use tiny_skia::{BlendMode, IntSize, Mask, Paint, Path, PathBuilder, Pixmap, PixmapMut, PixmapPaint, PremultipliedColor, Rect, Transform};
use helium_core::{Color, GlobalId, Position, Rgba, Size};
use super::View;

#[derive(Default)]
pub struct TextView{
    id: GlobalId,
    position: Position,
    size: Size,
    foreground_color: Color<Rgba>,
    text: String,
}

// TODO add background color
impl TextView {
    pub fn new(text: &str) -> Self {
        Self{
            text: text.to_owned(),
            ..Default::default()
        }
    }
}

impl View for TextView {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn color(&self) -> &Color<Rgba> {
        &self.foreground_color
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size
    }

    fn set_id(&mut self, id: GlobalId) {
        self.id = id
    }

    fn render(&self, pixmap: &mut Pixmap) {
        pixmap.fill(tiny_skia::Color::WHITE);
        let bytes = include_bytes!("../../../assets/fonts/Inter/static/Inter-Bold.ttf") as &[u8];
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();
        let font_size = 56.0;

        let mut x_pos = 0;
        
        // Draw each character onto a pixmap then 
        // draw that pixmap onto the root pixmap
        for c in self.text.chars() {
            let (metrics,bitmap) = font.rasterize(c,font_size);

            // Skip spaces to avoid panicking
            if metrics.width == 0 {
                // x_pos += metrics.advance_width;
                continue;
            }

            let size = IntSize::from_wh(metrics.width as u32, metrics.height as u32).unwrap();
            let mut colors = vec![];
            for a in bitmap.iter() {
                colors.push(0);
                colors.push(0);
                colors.push(0);
                colors.push(*a);
            }
            let glyph_pixmap = Pixmap::from_vec(colors,size).unwrap();
            
            pixmap.draw_pixmap(
                0,
                0,
                glyph_pixmap.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                None,
            );
        }

        pixmap.save_png("../temp/text.png").unwrap()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn render(){
        let view = TextView::new("hello world");
        let mut pixmap = Pixmap::new(250,250).unwrap();
        view.render(&mut pixmap);
    }
}