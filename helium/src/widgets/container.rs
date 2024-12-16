use nanoid::nanoid;

use super::WidgetBody;
use crate::{
    app::events::Event,impl_style, layout::{BlockLayout, Layout}, surface::rect::RectSurface, widgets::Widget
};
use helium_core::color::Color;

/// A container [`Widget`] that wraps its child
pub struct Container<W> {
	id:String,
    color: Color,
    child: W, // TODO make this a generic
	layout:BlockLayout,
	corner_radius:u32
}

impl<W> Container<W> 
where W:Widget {
    pub fn new(child:W) -> Self {
        Container {
			id:nanoid!(),
			layout:BlockLayout::new(0),
            color: Color::Rgb(255, 255, 255),
            child,
			corner_radius:0
        }
    }

	pub fn padding(mut self,padding:u32) -> Self{
		self.layout.padding(padding);
		self
	}

	pub fn corner_radius(mut self,corner_radius:u32) -> Self{
		self.corner_radius = corner_radius;
		self
	}

	impl_style!();

}

impl<W> Widget for Container<W>
where W:Widget {
    fn build(&self) -> WidgetBody {
        let surface = Box::new(RectSurface {
            color: self.color.clone(),
			corner_radius:self.corner_radius,
            ..Default::default()
        });


		WidgetBody {
			id:self.id.clone(),
            surface,
            layout:Box::new(self.layout),
            children: vec![Box::new(self.child.build())],
            ..Default::default()
        }
    }
}
