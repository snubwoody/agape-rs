/// Manages resources
#[derive(Default)]
pub struct ResourceManager{
	buffers:Vec<wgpu::Buffer>,
	texture:Vec<wgpu::Texture>,
	samplers:Vec<wgpu::Sampler>,
	bind_groups:Vec<wgpu::BindGroup>
}

impl ResourceManager {
	pub fn new() -> Self{
		Self::default()
	}

	/// Add a resource and get back the index of that resource
	/// 
	/// # Example
	/// 
	/// ```
	/// use helium::surface::ResourceManager;
	/// let mut resources = ResourceManager::new();
	/// resources.add_buffer()
	/// ```
	pub fn add_buffer(
		&mut self,
		label:&str,
		size:u64,
		usage:wgpu::BufferUsages,
		device:&wgpu::Device
	) -> usize{

		let buffer = device.create_buffer(&wgpu::BufferDescriptor{
			label: Some(label),
			usage:wgpu::BufferUsages::VERTEX,
			size,
			mapped_at_creation:false
		});

		0
	}

	pub fn write_buffer(&self){
		todo!()
	}

	pub fn write_texture(&self){
		todo!()
	}
}