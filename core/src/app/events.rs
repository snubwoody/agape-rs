use crate::{utils::Position, widgets::WidgetTree};
use winit::{dpi::PhysicalPosition, event::{ElementState, MouseButton, WindowEvent}};

pub enum EventFunction {
	OnClick(Box<dyn Fn()>),
	OnHover(Box<dyn Fn()>),
}

/// Manages User interaction events
pub struct EventManager{
	cursor_pos:Position
}

impl EventManager {
	pub fn new() -> Self{
		Self { cursor_pos: Position::new(0, 0) }
	}

	pub fn handle_events(&mut self,widget_tree:&mut WidgetTree,event:WindowEvent){
		match event{
			WindowEvent::CursorMoved { position,.. } => {
				self.cursor_pos = position.into();
				self.handle_hover_event(widget_tree, position)
			},
			WindowEvent::MouseInput { state, button,.. } => {
				self.handle_mouse_press(widget_tree,state, button)
			},
			_ => {}
		}
	}

	pub fn handle_mouse_press(&self,widget_tree:&mut WidgetTree,state:ElementState,button:MouseButton) {
		for (_,widget) in  widget_tree.widgets.iter_mut().enumerate(){
			let bounds = widget.get_bounds();
			if bounds.within(&self.cursor_pos){
				for (_,event_function) in widget.events.iter().enumerate(){
					match event_function {
						EventFunction::OnClick(func) => func(),
						_ => {}
					}
				}
			}
		}
	}

	pub fn handle_hover_event(&self,widget_tree:&mut WidgetTree,position:PhysicalPosition<f64>){
		for (_,widget) in  widget_tree.widgets.iter_mut().enumerate(){
			let bounds = widget.get_bounds();
			if bounds.within(&self.cursor_pos){
				for (_,event_function) in widget.events.iter().enumerate(){
					match event_function {
						EventFunction::OnHover(func) => func(),
						_ => {}
					}
				}
			}
		}
	}
}