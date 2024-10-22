use super::{Widget, WidgetBody, WidgetState};
use crate::app::events::Event;
use crate::color::Color;
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::utils::Size;

// TODO change size to u32
/// A simple rectangle
pub struct Rect{
    pub width: f32,
    pub height: f32,
    pub color: Color,
	pub events: Vec<Event>,
	pub state: WidgetState,
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
			events: Vec::new(),
			state: WidgetState::Default,
        }
    }

	fn snapshot(&self) -> Rect{
		Self { 
			width: self.width, 
			height: self.height, 
			color: self.color.clone(), 
			events: vec![],
			state: WidgetState::Default,
		}
	}

	/// This function gets called everytime the widgets state is changed.
	fn handle_state_changes(&mut self){		
		let mut state = self.snapshot();
		match self.state {
			WidgetState::Pressed => {
				for event in self.events.iter_mut(){
					match event {
						Event::OnClick(func) => func(),
						_ => {}
					}
				}
			},
			WidgetState::Hovered => {
				for event in self.events.iter_mut(){
					match event {
						Event::OnClick(func) => func(),
						_ => {}
					}
				}
			},
			WidgetState::Default=>{},
			_ => {}
		}
	}

	pub fn on_click(mut self, event: impl FnMut() + 'static ) -> Self {
		self.events.push(Event::OnClick(Box::new(event)));
		self
	}

	pub fn on_hover(mut self, event: impl FnMut() + 'static ) -> Self {
		self.events.push(Event::OnHover(Box::new(event)));
		self
	}
}
// TODO maybe add a widget state enum to represent the different states
// Then maybe somehow copy the user code to the function on_hover

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

	fn change_state(&mut self,state:WidgetState) {
		self.state = state;
		self.handle_state_changes();
	}
}

