mod circle;
mod rect;
mod text;
mod image;
pub use image::Image;
pub use circle::Circle;
pub use rect::Rect;
pub use text::Text;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Rect(Rect),
    Circle(Circle),
    Text(Text),
    Image(Image),
}

pub trait IntoPrimitive {
    fn into_primitive(self) -> Primitive;
}
