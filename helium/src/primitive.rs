use crate::{resources::ResourceManager, surface::Surface};

// TODO impl from
pub enum Primitiv {
    Text(&'static str),
    Image(image::DynamicImage),
    Icon,
    Rect,
    Circle,
}


pub trait Primitive {
	fn build(&self,resources: &mut ResourceManager,device: &wgpu::Device) -> impl Surface;
}

pub struct RectPrimitive{
	id: String,
	width: u32,
	height: u32,
	corner_radius: u32
}

impl RectPrimitive {
	pub fn new(id: &str) -> RectPrimitive{
		RectPrimitive { 
			id:id.to_string(), 
			width: 0, 
			height: 0, 
			corner_radius: 0 
		}
	}


	pub fn width(mut self,width:u32) -> Self{
		self.width = width;
		self
	}

	pub fn height(mut self,height:u32) -> Self{
		self.height = height;
		self
	}

	pub fn corner_radius(mut self,corner_radius:u32) -> Self{
		self.corner_radius = corner_radius;
		self
	}
}

