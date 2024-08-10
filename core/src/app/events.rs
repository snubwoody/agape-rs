use crate::widgets::WidgetTree;
use winit::{dpi::PhysicalPosition, event::{ElementState, MouseButton, WindowEvent}};

/*  TODO stick thinking of a way to implement events
	I need to be able to pass variables down to the functions
	and I also need to have access to the widget itself, which isn't 
	too hard but I also need to be able to change properties like
	size which live on the widget body so I don't know if that'll
	even work because the body is built after the widget.
*/
pub enum EventFunction {
	OnClick(Box<dyn Fn()>),
	OnHover(Box<dyn Fn()>),
}

pub struct EventManager;

impl EventManager {
	pub fn handle_events(&self,widget_tree:&mut WidgetTree,event:WindowEvent){
		match event{
			WindowEvent::CursorMoved { position,.. } => {
				self.handle_hover_event(widget_tree, position)
			},
			WindowEvent::MouseInput { state, button,.. } => {
				self.handle_mouse_press(state, button)
			},
			_ => {}
		}
	}

	pub fn handle_mouse_press(&self,state:ElementState,button:MouseButton) {

	}

	pub fn handle_hover_event(&self,widget_tree:&mut WidgetTree,position:PhysicalPosition<f64>){
		
		for (_,widget) in  widget_tree.widgets.iter_mut().enumerate(){
			let bounds = widget.get_bounds();
			let cursor_pos = [position.x as i32,position.y as i32];
			if bounds.within(cursor_pos){
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