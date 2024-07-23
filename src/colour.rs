use super::map;

pub fn rgb(r:i32,g:i32,b:i32) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	return [red,green,blue,1.0]
}

pub fn rgba(r:i32,g:i32,b:i32,a:i32) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	let alpha = map(a as f32, [0.0,100.0], [0.0,1.0]);
	return [red,green,blue,alpha]
}
