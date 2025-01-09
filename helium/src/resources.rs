/// Manages resources
#[derive(Default,Debug)]
pub struct ResourceManager {
    buffers: Vec<wgpu::Buffer>,
    textures: Vec<wgpu::Texture>,
    samplers: Vec<wgpu::Sampler>,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a buffer returns the index of that resource
    pub fn add_buffer(
        &mut self,
        label: &str,
        size: u64,
        usage: wgpu::BufferUsages,
        device: &wgpu::Device,
    ) -> usize {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            usage,
            size,
            mapped_at_creation: false,
        });

        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

    /// Creates a buffer returns the index of that resource
    pub fn add_vertex_buffer(
        &mut self,
        label: &str,
        size: u64,
        device: &wgpu::Device,
    ) -> usize {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            usage:wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size,
            mapped_at_creation: false,
        });

        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

    pub fn add_uniform(
        &mut self,
        label: &str,
        size: u64,
        device: &wgpu::Device,
    ) -> usize {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            usage:wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size,
            mapped_at_creation: false,
        });

        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

	/// Get a buffer 
	pub fn buffer(&self,index:usize) -> Option<&wgpu::Buffer>{
		self.buffers.get(index)
	}

    pub fn write_buffer(&self) {
        todo!()
    }

    pub fn write_texture(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use winit::{
		event_loop::EventLoopBuilder, 
		platform::windows::EventLoopBuilderExtWindows
	};

    async fn setup() -> wgpu::Device {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .unwrap();

        let window = winit::window::WindowBuilder::new()
			.with_visible(false)
            .build(&event_loop)
            .unwrap();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, _) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device/Queue"),
                    required_features: wgpu::Features::empty(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        device
    }

    #[async_std::test]
    async fn buffer_creation(){
		let device = setup().await;

		let mut resources = ResourceManager::new();
		let a = resources.add_buffer("Buffer",12,wgpu::BufferUsages::VERTEX,&device);
		let b = resources.add_vertex_buffer("Vertex Buffer",102,&device);
		let c = resources.add_uniform("Uniform Buffer",12,&device);

		resources.buffer(a).unwrap();
		resources.buffer(b).unwrap();
		resources.buffer(c).unwrap();
	}
}
