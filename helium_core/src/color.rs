use crate::map;
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
#[derive(Debug,Clone,PartialEq, Eq)]
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
				// Map invalid colors to white
				let color = Color::hex_to_rgba(&color).unwrap_or([255,255,255,100]);
				color
			}
		}
	}

	/// Convert a hex color to an rgba color.
	pub fn hex_to_rgba(hex:&str) -> Result<[u8;4],String>{
		if hex.chars().nth(0) != Some('#'){
			return Err("Invalid hex code: missing # at start of hex".into())
		}
		
		let hex_code = hex.strip_prefix("#").unwrap();

		if hex_code.len() != 6 {
			return Err("Invalid hex code: Hex colors should be 6 characters in length".into());
		}

		let (red,green,blue) = (
			&hex_code[0..2],
			&hex_code[2..4],
			&hex_code[4..6],
		);

		let r = u8::from_str_radix(red, 16).map_err(|err|{format!("Failed to parse hex code:{err}")})?;
		let g = u8::from_str_radix(green, 16).map_err(|err|{format!("Failed to parse hex code:{err}")})?;
		let b = u8::from_str_radix(blue, 16).map_err(|err|{format!("Failed to parse hex code:{err}")})?;

		Ok([r,g,b,100])
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



#[cfg(test)]
mod test{
	fn test_valid_hex_colors(){

	}

	/// Check if colors colors are clamped from 0 - 255 
	fn test_color_overflow(){

	}
}