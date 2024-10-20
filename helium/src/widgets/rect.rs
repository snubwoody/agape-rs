use std::fmt::Debug;
use super::{Widget, WidgetBody};
use crate::app::events::{Event, Interactive};
use crate::color::Color;
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::utils::{Position, Size};

#[derive(Debug)]
/// A simple rectangle
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub hover_func: Event<Self>,
	pub events: Vec<Event<Self>>
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
            hover_func:Event::OnHover(Box::new(|_|{})),
			events: Vec::new()
        }
    }

    pub fn on_hover(mut self, event: impl FnMut(&mut Rect) + 'static ) -> Self {
		self.events.push(Event::OnHover(Box::new(event)));
        self
    }

    pub fn on_click(mut self, event: impl FnMut(&mut Rect) + 'static ) -> Self {
		self.events.push(Event::OnClick(Box::new(event)));
        self
    }

	/// Make a 'copy' of the [`Widget`] to pass as the state.
	fn snapshot(&self) -> Self{
		Rect::new(self.width, self.height, self.color.clone())
	}

	fn update(&mut self,state:&Rect){
		self.color = state.color.clone();
		self.width = state.width;
		self.height = state.height;
	}
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
}

impl Interactive for Rect {
	fn handle_hover(&mut self,cursor_pos:Position) {
		let body = self.build();
		let bounds = body.surface.get_bounds();
		let mut state = self.snapshot();

		if bounds.within(&cursor_pos){
			for event in self.events.iter_mut(){
				match event {
					Event::OnHover(func) => func(&mut state),
					_ => {}
				}
			}
		}
		self.update(&state);
	}

	fn handle_click(&mut self,cursor_pos:Position) {
		let body = self.build();
		let bounds = body.surface.get_bounds();
		let mut state = self.snapshot();

		if bounds.within(&cursor_pos){
			for event in self.events.iter_mut(){
				match event {
					Event::OnClick(func) => func(&mut state),
					_ => {}
				}
			}
		}
		self.update(&state);
	}
}
