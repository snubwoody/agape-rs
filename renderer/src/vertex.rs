#[repr(C)]
#[derive(Debug,Clone,Copy,bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
	pub position:[f32;2],
	pub color: [f32;4]	
}

impl Vertex {
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
					format: wgpu::VertexFormat::Float32x3
				}
			]
		}
	}
}