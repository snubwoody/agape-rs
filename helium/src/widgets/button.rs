use nanoid::nanoid;

use crate::{
	app::events::Signal, color::Color, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::WidgetBody
};
use super::{text::Text, Widget};
use crate::app::events::Event;

/// A simple button.
pub struct Button{
	id:String,
	text:String,
	color:Color,
	padding:u32,
	width: WidgetSize,
	height: WidgetSize,
	events: Vec<Event>
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			id:nanoid!(),
			text:text.into(), 
			color:Color::Rgb(255, 255, 255),
			padding:12,
			width: WidgetSize::Fit,
			height:WidgetSize::Fit,
			events:Vec::new()
		}
	}

	pub fn color(mut self,color:Color) -> Self {
		self.color = color;
		self
	}

	pub fn padding(mut self, padding:u32) -> Self {
		self.padding = padding;
		self
	}

	pub fn width(mut self, width:f32) -> Self{
		self.width = WidgetSize::Fixed(width);
		self
	}

	pub fn height(mut self, height:f32) -> Self{
		self.height = WidgetSize::Fixed(height);
		self
	}

	pub fn fill(mut self) -> Self{
		self.width = WidgetSize::Fill;
		self
	}

	pub fn on_click(mut self, event: impl FnMut() + 'static ) -> Self {
		self.events.push(Event::OnClick(Box::new(event)));
		self
	}

	pub fn on_hover(mut self, event: impl FnMut() + 'static ) -> Self {
		self.events.push(Event::OnHover(Box::new(event)));
		self
	}

}

impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface::new(0.0, 0.0, 200.0, 70.0, self.color.clone())
		);

		let layout = Layout::Block{ padding: self.padding };

		let text_body = Text::new(&self.text).build();

		let intrinsic_size = IntrinsicSize{
			width:self.width,
			height:self.height
		};

		WidgetBody { 
			id:self.id.clone(),
			surface,
			layout,
			intrinsic_size,
			children: vec![
				Box::new(text_body)
			],
			..Default::default()
		}
	}

	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
		vec![]
	}

	fn process_signal(&mut self,signal:&Signal) {
		match signal {
			Signal::Click(id) =>{
				if id == &self.id{
					for event in self.events.iter_mut(){
						match event {
							Event::OnClick(func) => func(),
							_ => {}
						}
					}
				}
			}
			Signal::Hover(id) => {
				if id == &self.id{
					for event in self.events.iter_mut(){
						match event {
							Event::OnHover(func)=> func(),
							_ => {}
						}
					}
				}
			}
		}
	}

}