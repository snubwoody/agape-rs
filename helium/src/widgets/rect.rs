use std::fmt::Debug;
use super::{Widget, WidgetBody, WidgetState};
use crate::app::events::{Event};
use crate::color::{Color, BLACK, RED};
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::utils::Size;

#[derive(Debug)]
/// A simple rectangle
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub color: Color,
	pub events: Vec<Event<Self>>,
	pub state: WidgetState
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
			events: Vec::new(),
			state: WidgetState::Default
        }
    }

	fn snapshot(&self) -> Rect{
		Self { 
			width: self.width, 
			height: self.height, 
			color: self.color.clone(), 
			events: vec![],
			state: WidgetState::Default
		}
	}

	/// This function gets called everytime the widgets state is changed.
	fn handle_state_changes(&self){
		match self.state {
			WidgetState::Pressed => {dbg!("I was pressed");},
			WidgetState::Default=>{},
			_ => {}
		}
	}

	fn update(&mut self,state:&Self){
		self.width = state.width;
		self.height = state.height;
		self.color = state.color.clone();
	}

	pub fn on_hover(mut self, event: impl FnMut(&mut Rect) + 'static ) -> Self {
		self.events.push(Event::OnHover(Box::new(event)));
		self
	}

	pub fn on_click(mut self, event: impl FnMut(&mut Rect) + 'static ) -> Self {
		self.events.push(Event::OnClick(Box::new(event)));
		self
	}

	pub fn on_press(mut self, event: impl FnMut(&mut Rect) + 'static ) -> Self {
		self.events.push(Event::OnPress(Box::new(event)));
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

	// fn handle_hover(&mut self,cursor_pos:crate::utils::Position) {
	// 	let body = self.build();
	// 	let bounds = body.surface.get_bounds();
	// 	let mut state = self.snapshot();

	// 	if bounds.within(&cursor_pos){
	// 		for event in self.events.iter_mut(){
	// 			match event {
	// 				crate::app::events::Event::OnHover(func) => func(&mut state),
	// 				_ => {}
	// 			}
	// 		}
	// 	}
	// 	self.update(&state);
	// }

	fn change_state(&mut self,state:WidgetState) {
		self.state = state;
		self.handle_state_changes();
		dbg!(&state);
	}

	// fn handle_click(&mut self,cursor_pos:crate::utils::Position) {
	// 	let body = self.build();
	// 	let bounds = body.surface.get_bounds();
	// 	let mut state = self.snapshot();

	// 	if bounds.within(&cursor_pos){
	// 		for event in self.events.iter_mut(){
	// 			match event {
	// 				crate::app::events::Event::OnClick(func) => func(&mut state),
	// 				_ => {}
	// 			}
	// 		}
	// 	}
		
	// 	self.update(&state);
	// }

	// fn handle_press(&mut self,cursor_pos:crate::utils::Position) {
	// 	let body = self.build();
	// 	let bounds = body.surface.get_bounds();
	// 	let mut state = self.snapshot();

	// 	if bounds.within(&cursor_pos){
	// 		for event in self.events.iter_mut(){
	// 			match event {
	// 				crate::app::events::Event::OnPress(func) => func(&mut state),
	// 				_ => {}
	// 			}
	// 		}
	// 	}
	// 	self.update(&state);
	// }
}

