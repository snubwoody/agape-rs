use super::{WidgetBody,Widget};

pub enum FlexDirection {
	Vertical,
	Horizontal
}

pub struct Flex{
	order:Vec<u8>,
	direction:FlexDirection,
	children:Vec<Box<dyn Widget>>
}

impl Widget for Flex {
	fn build(&self) -> WidgetBody {
		let children = self.children.iter().map(|widget|{
			Box::new(widget.build())
		}).collect();

		WidgetBody{
			children,
			..Default::default()
		}
	}
}