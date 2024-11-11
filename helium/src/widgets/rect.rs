use super::{Widget, WidgetBody};
use crate::app::events::{Event, Signal};
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
    pub width: f32,
    pub height: f32,
    pub color: Color,
	pub events: Vec<Event>,
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
			id:nanoid!(),
            width,
            height,
            color,
			events: Vec::new(),
        }
    }

	impl_events!();
}

impl Widget for Rect {
    fn build(&self) -> WidgetBody {
        let layout = Layout::Block { padding: 0 };
        let surface = Box::new(RectSurface {
            size: Size::new(self.width as f32, self.height as f32),
            color: self.color.clone(),
            ..Default::default()
        });

        WidgetBody {
			id:self.id.clone(),
            surface,
            layout,
            children: vec![],
            intrinsic_size: IntrinsicSize {
                width: WidgetSize::Fixed(self.width),
                height: WidgetSize::Fixed(self.height),
            },
            ..Default::default()
        }
    }

	fn process_signal(&mut self,signal:&Signal) {
		match signal {
			Signal::Click(id) =>{
				if id == &self.id{
					for event in self.events.iter_mut(){
						match event {
							Event::OnClick(func) => func(),
							_ => {}
						}
					}
				}
			}
			Signal::Hover(id) => {
				if id == &self.id{
					for event in self.events.iter_mut(){
						match event {
							Event::OnHover(func)=> func(),
							_ => {}
						}
					}
				}
			}
		}
	}
}

