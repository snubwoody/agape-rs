use crystal::{BlockLayout, Layout};
use nanoid::nanoid;
use super::WidgetBody;
use crate::{impl_style, surface::rect::RectSurface, widgets::Widget};
use helium_core::color::Color;

/// A container [`Widget`] that wraps its child
pub struct Container<W> {
    id: String,
    color: Color,
    child: W, // TODO make this a generic
    corner_radius: u32,
	padding:u32
}

impl<W> Container<W>
where W: Widget{
    pub fn new(child: W) -> Self {
        Container {
            id: nanoid!(),
            color: Color::Rgb(255, 255, 255),
            child,
            corner_radius: 0,
			padding:0
        }
    }

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    impl_style!();
}

impl<W> Widget for Container<W>
where W: Widget {
    fn build(&self) -> (WidgetBody,Box<dyn Layout>) {
        let surface = Box::new(RectSurface {
            color: self.color.clone(),
            corner_radius: self.corner_radius,
            ..Default::default()
        });

		let (child_body,child_layout) = self.child.build();
		
        let body = WidgetBody{
			id: self.id.clone(),
            surface,
            children: vec![Box::new(child_body)],
            ..Default::default()
        };
		
		let mut layout = BlockLayout::new(child_layout);
		layout.id = body.id.clone();
		layout.padding = self.padding;

		(body,Box::new(layout))
    }

	fn update(&mut self) {
		self.child.update();
	}
}
