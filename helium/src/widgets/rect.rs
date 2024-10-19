use std::fmt::Debug;

use wgpu::hal::auxil::db;
use winit::event::WindowEvent;

use super::{Widget, WidgetBody};
use crate::app::events::EventFunction;
use crate::color::{Color, TEAL};
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::utils::{Position, Size};

#[derive(Debug)]
/// A simple rectangle
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub hover_func: EventFunction<Self>
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
            hover_func:EventFunction::OnHover(Box::new(|_|{}))
        }
    }

    pub fn on_hover(mut self, event: impl FnMut(&mut Rect) + 'static ) -> Self {
        self.hover_func = EventFunction::OnHover(Box::new(event));
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

	fn run_events(&mut self,event:&WindowEvent) {
		let body = self.build();
		let bounds = body.surface.get_bounds();
		let mut state = self.snapshot();

		match event {
			WindowEvent::CursorMoved { position,.. } => {
				let cursor_pos = Position::from(*position);
				if bounds.within(&cursor_pos){
					self.hover_func.run(&mut state);
				}
			},
			_ => {}
		}
		self.update(&state);
	}

    fn get_children(self: Box<Self>) -> Vec<Box<dyn Widget>> {
        vec![]
    }
}
