use crate::text::TextSurface;

#[derive(Debug)]
pub struct Text{
	surface:TextSurface,
	text:String,
	font_size:u8
}

impl Text {
	pub fn new(x:i32,y:i32,text:&str,colour:&str,font_size:u8) -> Self{
		let surface = TextSurface::new(x, y, text, colour, font_size);
		Self { surface, text:text.into(), font_size }	
	}
}
