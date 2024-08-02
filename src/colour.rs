use crate::utils::map;

pub fn rgb(r:u8,g:u8,b:u8) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	return [red,green,blue,1.0]
}

pub fn rgba(r:u8,g:u8,b:u8,a:u8) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	let alpha = map(a as f32, [0.0,100.0], [0.0,1.0]);
	return [red,green,blue,alpha]
}


/* // TODO
enum Colour{
	Hex(String),
	Rgb(u8,u8,u8),
	Rgba(u8,u8,u8,u8),
} */
