use crate::utils::map;

pub const BLACK:Colour = Colour::Rgb(0, 0, 0);
pub const WHITE:Colour = Colour::Rgb(255, 255, 255);
pub const AMBER:Colour = Colour::Rgb(245, 158, 11);
pub const GREEN:Colour = Colour::Rgb(34, 197, 94);
pub const BLUE:Colour = Colour::Rgb(0, 0, 254);
pub const RED:Colour = Colour::Rgb(255, 10, 94);
pub const TEAL:Colour = Colour::Rgb(20, 184, 166);
pub const INDIGO:Colour = Colour::Rgb(99, 102, 241);
pub const PINK:Colour = Colour::Rgb(236, 72, 153);

// TODO start changing to color instead
/// Represents a color.
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum Colour{
	Rgb(u8,u8,u8),
	Rgba(u8,u8,u8,u8),
	Hex(String)
} 

// TODO impl From
impl Colour {
	/// Parse any type of coulour to rgba values
	pub fn to_rgba(&self) -> [u8;4] {		
		match self {
			Self::Rgb(r,g,b) => [*r,*g,*b,100],
			Self::Rgba(r,g,b,mut a) => {
				if a > 100 {a = 100}
				[*r,*g,*b,a]
			},
			Self::Hex(colour) => {
				self.hex_to_rgba(&colour)
			}
		}
	}

	/// Convert a hex color to an rgba color.
	fn hex_to_rgba(&self,hex:&str) -> [u8;4] {
		// FIXME handle the errors
		let (r,g,b) = (
			&hex[0..2],
			&hex[2..4],
			&hex[4..6],
		);

		let r = u8::from_str_radix(r, 16).unwrap();
		let g = u8::from_str_radix(g, 16).unwrap();
		let b = u8::from_str_radix(b, 16).unwrap();

		[r,g,b,100]
	}

	/// Normalize the colours to a 0 - 1 scale.
	pub fn normalize(&self) -> [f32;4] {
		let rgba = self.to_rgba();

		let r = map(rgba[0] as f32, [0.0,255.0], [0.0,1.0]);
		let g = map(rgba[1] as f32, [0.0,255.0], [0.0,1.0]);
		let b = map(rgba[2] as f32, [0.0,255.0], [0.0,1.0]);
		let a = map(rgba[3] as f32, [0.0,100.0], [0.0,1.0]);
		[r,g,b,a]
	}
}

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


