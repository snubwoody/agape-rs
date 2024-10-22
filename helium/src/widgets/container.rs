use crate::{
	 color::Color, impl_interative, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::Widget
};
use super::WidgetBody;

/// A container [`Widget`] that wraps its child
pub struct Container{
	pub padding:u32,
	pub color:Color,
	pub child:Box<dyn Widget>
}

impl Container {
	pub fn new(child:impl Widget + 'static) -> Self{
		Container { 
			padding:0, 
			color:Color::Rgb(255, 255, 255), 
			child:Box::new(child)
		}
	}

	pub fn color(mut self,color:Color) -> Self{
		self.color = color;
		self
	}

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}
}

impl Widget for Container {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface{
				color:self.color.clone(),
				..Default::default()
			}
		);
		
		let layout = Layout::Block{ padding: self.padding };
		let child = self.child.build();

		WidgetBody{
			surface,
			layout,
			children:vec![Box::new(child)],
			..Default::default()
		}
	}

	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
		return vec![self.child];
	}

	fn get_children_ref(&self) -> Vec<&Box<dyn Widget>> {
		vec![&self.child]
	}
}
