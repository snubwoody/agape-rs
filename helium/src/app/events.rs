use std::fmt::Debug;

use crate::{utils::Position};
use winit::{dpi::PhysicalPosition, event::{ElementState, MouseButton, WindowEvent}};


pub enum EventFunction {
	OnClick(Box<dyn Fn()>),
	OnHover(Box<dyn Fn()>),
}

enum WidgetFunction {
	OnClick(fn())
}
