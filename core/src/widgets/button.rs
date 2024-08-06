use crate::{
	colour::Colour, 
	surface::Surface, 
	widgets::{
		WidgetBody,
		WidgetBuilder
	}
};

#[derive(Debug)]
pub struct Button{
	pub text:String
}

impl WidgetBuilder for Button {
	fn build(&self) -> WidgetBody {
		let surface = Surface::new(0, 0, 500, 500, Colour::Rgb(255, 25, 255));
		WidgetBody { surface }
	}
}