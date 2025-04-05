mod circle;
mod icon;
mod image;
mod rect;
mod text;
pub use circle::CircleSurface;
pub use icon::IconSurface;
pub use image::ImageSurface;
pub use rect::RectSurface;
pub use text::TextSurface;

#[derive(Debug, Clone, PartialEq)]
pub enum Surface {
    Rect(RectSurface),
    Circle(CircleSurface),
    Text(TextSurface),
    Image(ImageSurface),
    Icon(IconSurface),
}

pub trait IntoSurface {
    fn into_surface(self) -> Surface;
}

impl IntoSurface for Surface{
	fn into_surface(self) -> Surface {
		self
	}
}
