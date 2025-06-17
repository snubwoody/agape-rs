mod color;
mod position;
mod size;

pub use {size::*, color::*, position::*};

/// A global unique identifier
#[derive(Copy, Clone,PartialOrd, PartialEq,Eq,Debug,Ord,Hash)]
pub struct GlobalId(u32);

impl GlobalId {
    pub fn new() -> Self { 
        let id = rand::random();
        Self(id)
    }
}

impl Default for GlobalId {
    fn default() -> Self {
        Self::new()
    }
}
/// Map value from one range to another. Any overflow is clipped to the min or max
pub fn map(mut value: f32, input_range: [f32; 2], output_range: [f32; 2]) -> f32 {
    if value > input_range[1] {
        value = input_range[1]
    } else if value < input_range[0] {
        value = input_range[0]
    }

    let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
    let offset = input_range[0] * (scale) + output_range[0];

    value * scale + offset
}
