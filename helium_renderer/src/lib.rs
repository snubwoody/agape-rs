pub mod vertex;
mod rect;
mod resources;
mod error;
mod primitives;
use crystal::Position;
pub use error::Error;
use helium_core::{
	color::*, 
	Size
};
use primitives::RectShader;
use resources::ResourcePool;
use vertex::Vertex;
use winit::{
    dpi::PhysicalSize,
    window::Window,
};


pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
	/// The size of the `Window`.
	size: Size,
	shader:RectShader,
	window_bind_group:usize,
	window_buffer:usize,
	resources: ResourcePool
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = Size::from(window.inner_size());

        // Handle to wpgu for creating a surface and an adapter
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // Create the surface to draw on
        let surface = instance.create_surface(window).unwrap();

        // Handle to the graphics card
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // The device is an open connection to the graphics
        // card and the queue is a command buffer
        let (device, queue) = adapter
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

        let surface_caps = surface.get_capabilities(&adapter);

        // Get an sRGB texture format
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|s| s.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        // The surface configuration
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Configure the surface for presentation
        surface.configure(&device, &config);
		let mut resources = ResourcePool::new();
		
		let shader = RectShader::new(&device, &mut resources, config.format).unwrap();
		
		let window_buffer = resources.add_buffer_init(
			"Global window buffer", 
			bytemuck::cast_slice(&[window.inner_size().width as f32,window.inner_size().height as f32]), 
			wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
			&device
		);

		let window_bind_group = resources.add_bind_group(
			"Global window bind group", 
			&shader.window_layout(), 
			&device, 
			&[window_buffer], 
			&[], 
			&[]
		).unwrap();

		// FIXME return error

        Self {
            surface,
            device,
            queue,
            config,
            size,
			shader,
			window_buffer,
			window_bind_group,
			resources,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = Size::from(size);
        self.config.width = size.width;
        self.config.height = size.height;
        
		// Resize the surface with the window to keep the right scale
		self.resources.write_buffer(
			self.window_buffer, 
			0,
			bytemuck::cast_slice(&[self.size.width,self.size.height]), 
			&self.queue
		).unwrap();

		self.surface.configure(&self.device, &self.config);
    }

	pub fn render(&mut self){
		let instant = std::time::Instant::now();
		
		let output = self.surface.get_current_texture().unwrap(); // TODO maybe handle this error
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render encoder"),
			});


		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color {
						r: 1.0,
						g: 1.0,
						b: 1.0,
						a: 1.0,
					}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});

		

		self.draw_rect(&mut render_pass);
		// Drop the render pass because it borrows encoder mutably
		std::mem::drop(render_pass);

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		
		//dbg!(instant.elapsed());
	}

	pub fn draw_rect(
		&mut self,
		pass:&mut wgpu::RenderPass,
	){
		let device = &self.device;
		let size = Size::new(250.0, 250.0);
		let position = Position::new(0.0, 0.0);
		
		let vertices = Vertex::quad(size, position, RED);
	
		let vertex_buffer = self.resources.add_vertex_buffer_init(
			"Rect Vertex Buffer",
			bytemuck::cast_slice(&vertices),
			device,
		);
	
		let size_buffer = self.resources.add_uniform_init(
			"Rect Size Buffer",
			bytemuck::cast_slice(&[size.width, size.height]),
			device,
		);
	
		let position_buffer = self.resources.add_uniform_init(
			"Rect Position Buffer",
			bytemuck::cast_slice(&[position.x, position.y]),
			device,
		);
	
		let radius_buffer = self.resources.add_uniform_init(
			"Rect Corner Radius Buffer",
			bytemuck::cast_slice(&[0]),
			device,
		);
	
		let bind_group_index = self.resources.add_bind_group(
			"Rect Bind Group",
			self.shader.layout(),
			device,
			&[radius_buffer, size_buffer, position_buffer],
			&[],
			&[],
		).unwrap();

		let bind_group = self.resources.bind_group(bind_group_index).unwrap();

		let vertex_buffer = self.resources
            .buffer(vertex_buffer)
            .unwrap();

        let window_bind_group = self.resources
            .bind_group(self.window_bind_group)
            .unwrap();

        pass.set_pipeline(self.shader.pipeline());
        pass.set_bind_group(0, window_bind_group, &[]);
        pass.set_bind_group(1, bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..vertices.len() as u32, 0..1);
	}
	
}



#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn create_renderer(){

	}
}
