/// The bounds of any object that has a size 
/// and position
pub struct Bounds {
	pub x:[i32;2],
	pub y:[i32;2],
}

impl Bounds{
	/// Check if a [`Position`] is within the bounds
	pub fn within(&self,position:[i32;2]) -> bool {
		if 
			position[0] > self.x[0] && 
			position[0] < self.x[1] &&
			position[1] > self.y[0] &&
			position[1] < self.y[1] {
			return true;
		}

		false
	}
}


/// Map value from one range to another. Any overflow is clipped to the min or max
pub fn map(mut value:f32,input_range:[f32;2],output_range:[f32;2]) -> f32{
	if value > input_range[1]{
		value = input_range[1]
	}
	else if value < input_range[0] {
		value = input_range[0]
	}

	let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
	let offset = input_range[0]*(scale)+output_range[0];

	return  value * scale + offset;
}


