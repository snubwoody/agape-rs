use crate::{
	colour::Colour, 
	surface::Surface, 
	widgets::{
		WidgetBody,
		WidgetBuilder
	}
};

pub struct Button{
	text:String
}

impl WidgetBuilder for Button {
	fn build(&self) -> WidgetBody {
		let surface = Surface::new(0, 0, 500, 500, Colour::Rgb(255, 255, 255));
		WidgetBody { surface }
	}
}