/// A struct which hold all the vertex attributes ie. color
/// and position
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vertex{
	position: [i32;2], // TODO change to f32
	colour:[f32;4],
	uv:[f32;2],
}

impl Vertex {
	pub fn new(x:i32,y:i32,colour:[f32;4]) -> Self{
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a],
			uv:[1.0,1.0],
		}
	}

	pub fn new_with_texture(x:i32,y:i32,colour:[f32;4],texture_coords:[f32;2]) -> Self {
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a],
			uv:texture_coords,
		}
	}
}

implement_vertex!(Vertex,position,colour,uv);