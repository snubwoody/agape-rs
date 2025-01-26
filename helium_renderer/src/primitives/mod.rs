mod rect;
mod circle;
pub use rect::Rect;
pub use circle::Circle;

#[derive(Debug,Clone,PartialEq)]
pub enum Primitive {
	Rect(Rect),
	Circle(Circle),
}

pub trait IntoPrimitive{
	fn into_primitive(self) -> Primitive;
}