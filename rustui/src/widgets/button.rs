use crate::{
	colour::{Colour, BLACK}, 
	layout::{Constraint, IntrinsicSize, Layout, WidgetSize}, 
	surface::{rect::RectSurface, text::TextSurface}, 
	widgets::WidgetBody
};
use super::{text::Text, Widget};

#[derive(Debug)]
pub struct Button{
	pub text:String,
	pub colour:Colour,
	pub padding:u32,
	//events:Vec<EventFunction>
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			text:text.into(), 
			colour:Colour::Rgb(255, 255, 255),
			padding:0
			//events:Vec::new()
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self {
		self.colour = colour;
		self
	}

	pub fn padding(mut self, padding:u32) -> Self {
		self.padding = padding;
		self
	}

	/* pub fn on_hover(mut self,f:impl Fn() + 'static) -> Self {
		self.events.push(EventFunction::OnHover(Box::new(f)));
		self
	}

	pub fn on_click(mut self,f:impl Fn() + 'static) -> Self{
		self.events.push(EventFunction::OnClick(Box::new(f)));
		self
	} */
}

impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface::new(0.0, 0.0, 200, 70, self.colour.clone())
		);

		let layout = Layout::Block{ padding: self.padding };
		// FIXME

		let text_body = Text::new(&self.text).build();

		WidgetBody { 
			surface,
			layout,
			children: vec![
				Box::new(text_body)
			],
			..Default::default()
		}
	}

	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}