use std::fmt::Debug;

pub enum EventFunction<W> {
	OnClick(Box<dyn Fn(&mut W)>),
	OnHover(Box<dyn Fn(&mut W)>),
}

impl<W> EventFunction<W> {
	pub fn run(&self,widget:&mut W) {
		match self{
			Self::OnClick(func) => func(widget),
			Self::OnHover(func) => func(widget),
		}
	}
}

impl<W> Debug for EventFunction<W> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

struct EventHandler{

}

impl EventHandler {
	pub fn new() -> Self{
		todo!()
	}
}

