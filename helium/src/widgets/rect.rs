use winit::event::WindowEvent;

use super::{Widget, WidgetBody};
use crate::app::events::EventFunction;
use crate::color::Color;
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::utils::{Position, Size};

/// A simple rectangle
#[derive(Debug)]
pub struct Rect<'a> {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub on_hover: Option<EventFunction<Self>>,
}

impl<'a> Rect<'a> {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
            on_hover:None,
        }
    }

    pub fn on_hover(mut self, event: impl Fn(&Rect<'a>) + 'static ) -> Self {
        self.on_hover = Some(EventFunction::OnHover(Box::new(event)));
        self
    }
}

impl<'a> Widget for Rect<'a> {
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

	fn run_events(&mut self,event:winit::event::WindowEvent) {
		let body = self.build();
		let bounds = body.surface.get_bounds();

		match event {
			WindowEvent::CursorMoved { position,.. } => {
				let cursor_pos = Position::from(position);
				if bounds.within(&cursor_pos){
					match &self.on_hover {
						Some(EventFunction::OnHover(func)) => {func(self)}
						_ => {}
					}
				}
			},
			_ => {}
		}
	}

    fn get_children(self: Box<Self>) -> Vec<Box<dyn Widget>> {
        vec![]
    }
}
