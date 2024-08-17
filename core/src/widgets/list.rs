use crate::{layout::{self, Layout}, surface::Surface, text::TextSurface};

use super::{Widget, WidgetBody};

/// A simple text list
pub struct TextList{
	pub items:Vec<String>
}

impl TextList {
	pub fn new(items:Vec<&str>) -> Self{
		Self{
			items:items.iter().map(|text|text.to_string()).collect()
		}
	}

	fn build_list_item(&self,list_item:&String,index:usize) -> WidgetBody{
		let text_surface = TextSurface::new(list_item, "#000000", 16);
		let text_size = text_surface.get_size();
		let text_layout = Layout::SingleChild { width: text_size.0, height: text_size.1 };

		let text_body = WidgetBody{
			surface:Box::new(text_surface),
			layout:text_layout,
			..Default::default()
		};

		let style_surface = TextSurface::new(index.to_string().as_str(), "#000000", 16);
		let style_size = style_surface.get_size();
		let style_layout = Layout::SingleChild { width: style_size.0, height: style_size.1 };

		let style_body = WidgetBody{
			surface:Box::new(style_surface),
			layout:style_layout,
			..Default::default()
		};

		WidgetBody { 
			layout: Layout::Horizontal { spacing: 8, padding: 0 }, 
			children: vec![
				Box::new(style_body),
				Box::new(text_body),
			],
			..Default::default()
		}
	}
}

impl Widget for TextList {
	fn build(&self) -> WidgetBody {
		let mut bodies = Vec::new();
		let layout = Layout::Vertical { spacing: 12, padding: 0 };

		for (index,list_item) in self.items.iter().enumerate(){
			let body = self.build_list_item(list_item, index);
			bodies.push(Box::new(body))
		}

		WidgetBody{
			layout,
			children:bodies,
			..Default::default()
		}
	}
}