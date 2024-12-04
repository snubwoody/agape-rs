use nanoid::nanoid;

use super::WidgetBody;
use crate::{
    app::events::{Event, }, impl_events, impl_style, layout::Layout, surface::rect::RectSurface, widgets::Widget
};
use helium_core::color::Color;

/// A container [`Widget`] that wraps its child
pub struct Container {
	id:String,
    color: Color,
    child: Box<dyn Widget>,
    events: Vec<Event>,
	layout:Layout
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Container {
			id:nanoid!(),
			layout:Layout::new(),
            color: Color::Rgb(255, 255, 255),
            child: Box::new(child),
			events:vec![]
        }
    }

	impl_style!();

	impl_events!();
}

impl Widget for Container {
    fn build(&self) -> WidgetBody {
        let surface = Box::new(RectSurface {
            color: self.color.clone(),
            ..Default::default()
        });

        let child = self.child.build();

        WidgetBody {
			id:self.id.clone(),
            surface,
            layout:self.layout,
            children: vec![Box::new(child)],
            ..Default::default()
        }
    }
}
