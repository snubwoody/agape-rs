use nanoid::nanoid;

use super::WidgetBody;
use crate::{
    app::events::{Event, Signal}, impl_events, layout::Layout, surface::rect::RectSurface, widgets::Widget
};
use helium_core::color::Color;

/// A container [`Widget`] that wraps its child
pub struct Container {
	id:String,
    padding: u32,
    color: Color,
    child: Box<dyn Widget>,
    events: Vec<Event>,
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Container {
			id:nanoid!(),
            padding: 0,
            color: Color::Rgb(255, 255, 255),
            child: Box::new(child),
			events:vec![]
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

	impl_events!();
}

impl Widget for Container {
    fn build(&self) -> WidgetBody {
        let surface = Box::new(RectSurface {
            color: self.color.clone(),
            ..Default::default()
        });

        let layout = Layout::Block {
            padding: self.padding,
        };
        let child = self.child.build();

        WidgetBody {
			id:self.id.clone(),
            surface,
            layout,
            children: vec![Box::new(child)],
            ..Default::default()
        }
    }

    fn get_children(self: Box<Self>) -> Vec<Box<dyn Widget>> {
        vec![self.child]
    }

    fn get_children_ref(&self) -> Vec<&Box<dyn Widget>> {
        vec![&self.child]
    }

    fn process_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::Click(id) => {
                if id == &self.id {
                    for event in self.events.iter_mut() {
                        match event {
                            Event::OnClick(func) => func(),
                            _ => {}
                        }
                    }
                }
            }
            Signal::Hover(id) => {
                if id == &self.id {
                    for event in self.events.iter_mut() {
                        match event {
                            Event::OnHover(func) => func(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
