use super::{Widget, WidgetBody};
use crate::app::events::{Event, };
use crate::Color;
use crate::impl_events;
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::Size;
use nanoid::nanoid;

// TODO change size to u32
/// A simple rectangle
pub struct Rect{
	id:String,
    width: u32,
    height: u32,
    color: Color,
	events: Vec<Event>,
	intrinsic_size:IntrinsicSize
}

impl Rect {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        Self {
			id:nanoid!(),
            width,
            height,
            color,
			events: Vec::new(),
			intrinsic_size:IntrinsicSize::fixed(width, height)
        }
    }

	pub fn fill(mut self) -> Self{
		self.intrinsic_size.fill();
		self
	}

	pub fn fill_width(mut self) -> Self{
		self.intrinsic_size.fill_width();
		self
	}

	pub fn fill_height(mut self) -> Self{
		self.intrinsic_size.fill_height();
		self
	}

	impl_events!();
}

impl Widget for Rect {
    fn build(&self) -> WidgetBody {
        let surface = Box::new(RectSurface {
            size: Size::new(self.width as f32, self.height as f32),
            color: self.color.clone(),
            ..Default::default()
        });

        WidgetBody {
			id:self.id.clone(),
            surface,
            children: vec![],
            intrinsic_size:self.intrinsic_size,
            ..Default::default()
        }
    }

}

