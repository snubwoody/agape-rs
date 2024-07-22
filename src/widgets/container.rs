use std::fs;
use glium::{
	glutin::surface::WindowSurface, index, Display, Frame, Program, Surface, VertexBuffer
};
use winit::window::Window;
use crate::widgets::Widget;



pub struct Container<'a>{
	padding:i32, 
	child:&'a dyn Widget
}
