/// Manages resources
#[derive(Default)]
pub struct ResourceManager {
    buffers: Vec<wgpu::Buffer>,
    texture: Vec<wgpu::Texture>,
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

    #[test]
    fn buffer_creation() {}
}
