mod rect;
pub use rect::Rect;

#[derive(Debug)]
pub enum Primitive {
	Rect(Rect)
}

pub trait IntoPrimitive{
	fn into_primitive(self) -> Primitive;
}