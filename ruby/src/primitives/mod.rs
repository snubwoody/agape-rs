mod circle;
mod icon;
mod image;
mod rect;
mod text;
pub use circle::Circle;
pub use icon::Icon;
pub use image::Image;
pub use rect::RectSurface;
pub use text::Text;

#[derive(Debug, Clone, PartialEq)]
pub enum Surface {
    Rect(RectSurface),
}

pub trait IntoSurface {
    fn into_surface(self) -> Surface;
}

impl IntoSurface for Surface{
	fn into_surface(self) -> Surface {
		self
	}
}
