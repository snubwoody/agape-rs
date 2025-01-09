use std::fmt::format;

use crystal::Size;
use crate::error::Error;

/// Manages resources
#[derive(Default, Debug)]
pub struct ResourceManager {
    buffers: Vec<wgpu::Buffer>,
    textures: Vec<wgpu::Texture>,
    texture_views: Vec<wgpu::TextureView>,
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
    pub fn add_vertex_buffer(&mut self, label: &str, size: u64, device: &wgpu::Device) -> usize {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size,
            mapped_at_creation: false,
        });

        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

    pub fn add_uniform(&mut self, label: &str, size: u64, device: &wgpu::Device) -> usize {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size,
            mapped_at_creation: false,
        });

        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

    pub fn add_texture(&mut self, label: &str, size: Size, device: &wgpu::Device) -> usize {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size.width as u32,
                height: size.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.textures.push(texture);

        self.textures.len() - 1
    }

    /// Add a texture view of the texture at a specific index
    ///
    /// # Errors
    /// This function returns an error if the texture is not found
    pub fn add_texture_view(&mut self, index: usize) -> Result<usize, crate::error::Error> {
        let texture = self
            .texture(index)
            .ok_or_else(|| Error::NotFound(format!("Texture at index {index}")))?;

        let view = texture.create_view(&Default::default());

        self.texture_views.push(view);

        Ok(self.texture_views.len() - 1)
    }

	/// Add a texture sampler
	pub fn add_sampler(&mut self,label: &str,device: &wgpu::Device) -> usize{
		let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(label),
            ..Default::default()
        });

		self.samplers.push(texture_sampler);

		self.samplers.len() - 1
	}

	pub fn add_bind_group(
		&mut self, 
		label: &str, 
		layout:&wgpu::BindGroupLayout, 
		device: &wgpu::Device,
		buffers: &[usize],
		texture_views: &[usize],
		samplers: &[usize]
	) -> Result<usize,Error> {
		let mut entries = vec![]; 

		for i in buffers{
			let buffer = self.buffer(*i)
				.ok_or(Error::NotFound(format!("Buffer at index {i}")))?;

			entries.push(
				wgpu::BindGroupEntry {
                    binding: entries.len() as u32,
                    resource: buffer.as_entire_binding(),
                }	
			);
		}

		for i in texture_views{
			let texture_view = self.texture_view(*i)
				.ok_or(Error::NotFound(format!("Texture view at index {i}")))?;

			entries.push(
				wgpu::BindGroupEntry {
                    binding: entries.len() as u32,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                }	
			);
		}

		for i in samplers{
			let sampler = self.sampler(*i)
				.ok_or(Error::NotFound(format!("Sampler at index {i}")))?;

			entries.push(
				wgpu::BindGroupEntry {
                    binding: entries.len() as u32,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }	
			);
		}

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &layout,
            entries: &entries,
        });

		self.bind_groups.push(bind_group);

		Ok(self.bind_groups.len() - 1)
	}

    /// Get a `wgpu::Buffer`
    pub fn buffer(&self, index: usize) -> Option<&wgpu::Buffer> {
        self.buffers.get(index)
    }

    /// Get a `wgpu::Texture`
    pub fn texture(&self, index: usize) -> Option<&wgpu::Texture> {
        self.textures.get(index)
    }

    /// Get a `wgpu::Sampler`
    pub fn sampler(&self, index: usize) -> Option<&wgpu::Sampler> {
        self.samplers.get(index)
    }

    /// Get a `wgpu::TextureView`
    pub fn texture_view(&self, index: usize) -> Option<&wgpu::TextureView> {
        self.texture_views.get(index)
    }

    /// Get a `wgpu::BindGroup`
    pub fn bind_group(&self, index: usize) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(index)
    }

    /// Overwrite a buffer at a specific index, by replacing it.
    pub fn overwrite_buffer(&self, index: usize) {
        todo!()
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
    use winit::{event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows};

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
    async fn resource_creation() {
        let device = setup().await;

        let mut resources = ResourceManager::new();
        let a = resources.add_buffer("Buffer", 12, wgpu::BufferUsages::VERTEX, &device);
        let b = resources.add_vertex_buffer("Vertex Buffer", 102, &device);
        let c = resources.add_uniform("Uniform Buffer", 12, &device);
        let d = resources.add_texture("Texture", Size::default(), &device);

        resources.buffer(a).unwrap();
        resources.buffer(b).unwrap();
        resources.buffer(c).unwrap();
        resources.buffer(d).unwrap();
    }

	#[test]
	fn missing_texture_when_creating_texture_view(){
		let mut resources = ResourceManager::new();
		let res = resources.add_texture_view(0);

		assert!(matches!(res,Err(Error::NotFound(_))));
	}
}
