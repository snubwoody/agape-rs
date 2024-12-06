use nanoid::nanoid;

use super::WidgetBody;
use crate::{
    app::events::{Event, },impl_style, layout::Layout, surface::rect::RectSurface, widgets::Widget
};
use helium_core::color::Color;

/// A container [`Widget`] that wraps its child
pub struct Container {
	id:String,
    color: Color,
    child: Box<dyn Widget>,
	layout:Layout
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Container {
			id:nanoid!(),
			layout:Layout::new(),
            color: Color::Rgb(255, 255, 255),
            child: Box::new(child),
        }
    }

	impl_style!();

}

impl Widget for Container {
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
