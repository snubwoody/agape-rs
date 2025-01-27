mod circle;
mod rect;
mod text;
pub use circle::Circle;
pub use rect::Rect;
pub use text::Text;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Rect(Rect),
    Circle(Circle),
    Text(Text),
}

pub trait IntoPrimitive {
    fn into_primitive(self) -> Primitive;
}
