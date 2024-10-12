/// A struct which hold all the vertex attributes ie. color
/// and position
#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq,bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
	position: [f32;2], // TODO change to f32
	colour:[f32;4],
	uv:[f32;2],
}

impl Vertex {
	pub fn new(x:f32,y:f32,colour:[f32;4]) -> Self{
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a],
			uv:[1.0,1.0],
		}
	}

	pub fn new_with_texture(x:f32,y:f32,colour:[f32;4],texture_coords:[f32;2]) -> Self {
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a],
			uv:texture_coords,
		}
	}

	pub fn decription() -> wgpu::VertexBufferLayout<'static> {
		wgpu::VertexBufferLayout { 
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
			step_mode: wgpu::VertexStepMode::Vertex, 
			attributes: &[
				wgpu::VertexAttribute{
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x3
				},
				wgpu::VertexAttribute{
					offset: size_of::<[f32;3]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x3 // TODO Change this to include alpha channel
				}
			]
		}
	}
}