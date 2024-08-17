use crate::utils::map;

/// Deprecated
pub fn rgb(r:u8,g:u8,b:u8) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	return [red,green,blue,1.0]
}

/// Deprecated
pub fn rgba(r:u8,g:u8,b:u8,a:u8) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	let alpha = map(a as f32, [0.0,100.0], [0.0,1.0]);
	return [red,green,blue,alpha]
}


#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord)]
pub enum Colour{
	Rgb(u8,u8,u8),
	Rgba(u8,u8,u8,u8),
} 

// TODO impl From
impl Colour {
	pub fn to_rgba(self) -> [u8;4] {		
		match self {
			Self::Rgb(r,g,b) => [r,g,b,100],
			Self::Rgba(r,g,b,mut a) => {
				if a > 100 {
					a = 100}
				[r,g,b,a]
			},
		}
	}

	pub fn normalize(self) -> [f32;4] {
		let rgba = self.to_rgba();

		let r = map(rgba[0] as f32, [0.0,255.0], [0.0,1.0]);
		let g = map(rgba[1] as f32, [0.0,255.0], [0.0,1.0]);
		let b = map(rgba[2] as f32, [0.0,255.0], [0.0,1.0]);
		let a = map(rgba[3] as f32, [0.0,100.0], [0.0,1.0]);
		[r,g,b,a]
	}
}