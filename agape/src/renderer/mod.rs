mod image;
mod rect;
mod text;

use fontdue::{Font, FontSettings};
pub use image::draw_image;
pub use rect::draw_rect;
pub use text::{draw_text, text_size};

pub fn init_font() -> Font {
    let bytes = include_bytes!("../../fonts/Inter/static/Inter-Regular.ttf") as &[u8];
    Font::from_bytes(bytes, FontSettings::default()).unwrap()
}
