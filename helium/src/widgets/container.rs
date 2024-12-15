use nanoid::nanoid;

use super::WidgetBody;
use crate::{
    app::events::{Event, },impl_style, layout::Layout, surface::rect::RectSurface, widgets::Widget
};
use helium_core::color::Color;

/// A container [`Widget`] that wraps its child
pub struct Container<W> {
	id:String,
    color: Color,
    child: W, // TODO make this a generic
	layout:Layout
}

impl<W> Container<W> 
where W:Widget {
    pub fn new(child:W) -> Self {
        Container {
			id:nanoid!(),
			layout:Layout::new(),
            color: Color::Rgb(255, 255, 255),
            child
        }
    }

	impl_style!();

}

impl<W> Widget for Container<W>
where W:Widget {
    fn build(&self) -> WidgetBody {
        let surface = Box::new(RectSurface {
            color: self.color.clone(),
            ..Default::default()
        });

		WidgetBody {
			id:self.id.clone(),
            surface,
            layout:self.layout,
            children: vec![Box::new(self.child.build())],
            ..Default::default()
        }
    }
}
