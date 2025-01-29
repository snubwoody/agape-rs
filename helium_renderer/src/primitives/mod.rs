mod circle;
mod icon;
mod image;
mod rect;
mod text;
pub use circle::Circle;
pub use icon::Icon;
pub use image::Image;
pub use rect::Rect;
pub use text::Text;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Rect(Rect),
    Circle(Circle),
    Text(Text),
    Image(Image),
    Icon(Icon),
}

pub trait IntoPrimitive {
    fn into_primitive(self) -> Primitive;
}
