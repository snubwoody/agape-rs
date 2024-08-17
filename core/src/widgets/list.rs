use super::{Widget, WidgetBody};


pub enum ListStyle{
	Numbered,
	Bullet,
	CustomIcon,
	CustomString
}

/// A simple text list
pub struct TextList{
	items:Vec<String>
}

impl Widget for TextList {
	fn build(&self) -> super::WidgetBody {
		//for (i in )
		WidgetBody{
			..Default::default()
		}
	}
}