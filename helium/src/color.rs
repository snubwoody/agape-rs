use crate::utils::map;

pub const BLACK:Color = Color::Rgb(0, 0, 0);
pub const WHITE:Color = Color::Rgb(255, 255, 255);
pub const AMBER:Color = Color::Rgb(245, 158, 11);
pub const GREEN:Color = Color::Rgb(34, 197, 94);
pub const BLUE:Color = Color::Rgb(0, 0, 254);
pub const RED:Color = Color::Rgb(255, 10, 94);
pub const TEAL:Color = Color::Rgb(20, 184, 166);
pub const INDIGO:Color = Color::Rgb(99, 102, 241);
pub const PINK:Color = Color::Rgb(236, 72, 153);

/// Represents a color.
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum Color{
	Rgb(u8,u8,u8),
	Rgba(u8,u8,u8,u8),
	Hex(String)
} 

// TODO impl From
impl Color {
	/// Parse any type of coulour to rgba values
	pub fn to_rgba(&self) -> [u8;4] {		
		match self {
			Self::Rgb(r,g,b) => [*r,*g,*b,100],
			Self::Rgba(r,g,b,mut a) => {
				if a > 100 {a = 100}
				[*r,*g,*b,a]
			},
			Self::Hex(color) => {
				self.hex_to_rgba(&color)
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

	/// Normalize the colors to a 0 - 1 scale.
	pub fn normalize(&self) -> [f32;4] {
		let rgba = self.to_rgba();

		let r = map(rgba[0] as f32, [0.0,255.0], [0.0,1.0]);
		let g = map(rgba[1] as f32, [0.0,255.0], [0.0,1.0]);
		let b = map(rgba[2] as f32, [0.0,255.0], [0.0,1.0]);
		let a = map(rgba[3] as f32, [0.0,100.0], [0.0,1.0]);
		[r,g,b,a]
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::Rgb(255, 255, 255)
	}
}


