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

        let mut x_pos: i32 = 0;
        let baseline = 56;
        
        // Draw each character onto a pixmap then 
        // draw that pixmap onto the root pixmap
        for c in self.text.chars() {
            let (metrics,bitmap) = font.rasterize(c,font_size);

            // Skip spaces to avoid panicking
            if metrics.width == 0 {
                x_pos += metrics.advance_width as i32;
                continue;
            }
            
            // This will make every character lie on the baseline
            let mut y_pos: i32 = baseline - metrics.height as i32;
            dbg!(metrics,c);
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
                x_pos,
                y_pos,
                glyph_pixmap.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                None,
            );
            
            // Move the cursor to the next character
            // This is essentially letter spacing
            x_pos += metrics.advance_width.round() as i32;
        }

        pixmap.save_png("../temp/text.png").unwrap()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn text_rendering(){
        let view = TextView::new("hello world");
        let mut pixmap = Pixmap::new(500,500).unwrap();
        view.render(&mut pixmap);
    }
}