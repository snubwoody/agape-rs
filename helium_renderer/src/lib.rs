pub mod vertex;
pub mod rect;
mod resources;
mod error;
mod builders;
mod primitives;
use builders::BufferBuilder;
pub use error::Error;
use helium_core::{
	color::*, 
	Size
};
use primitives::RectShader;
use rect::Rect;
use resources::ResourcePool;
use vertex::Vertex;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    window::Window,
};


pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
	size: Size,
	shader:RectShader,
	window_bind_group:usize,
	window_buffer:usize,
	resources: ResourcePool,
	rect_renderer:RectRenderer,
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
            .unwrap(); // FIXME return these errors

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
		let rect_renderer = RectRenderer::new(&device, config.format);


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
			rect_renderer
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

		
		let rect = Rect::new(50.0, 50.0).color(RED);
		let rect_2 = Rect::new(150.0, 50.0).color(BLUE).position(150.0, 150.0);
		//self.rect_renderer.draw(&rect, &self.device, &mut self.resources, &mut render_pass);
		self.draw_rect(&mut render_pass,&rect);
		self.draw_rect(&mut render_pass,&rect_2);
		
		// Drop the render pass because it borrows encoder mutably
		std::mem::drop(render_pass);

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		
		//dbg!(instant.elapsed());
	}

	pub fn draw_rect(
		&mut self,
		pass:&mut wgpu::RenderPass,
		rect:&Rect,
	){
		let device = &self.device;
		
		let vertices = Vertex::quad(rect.size, rect.position, rect.color);
	
		let vertex_buffer = self.resources.add_vertex_buffer_init(
			"Rect Vertex Buffer",
			bytemuck::cast_slice(&vertices),
			device,
		);

		let buffer = BufferBuilder::new().init(&[2]);
	
		let size_buffer = self.resources.add_uniform_init(
			"Rect Size Buffer",
			bytemuck::cast_slice(&[rect.size.width, rect.size.height]),
			device,
		);
	
		let position_buffer = self.resources.add_uniform_init(
			"Rect Position Buffer",
			bytemuck::cast_slice(&[rect.position.x, rect.position.y]),
			device,
		);
	
		let radius_buffer = self.resources.add_uniform_init(
			"Rect Corner Radius Buffer",
			bytemuck::cast_slice(&[12.0]),
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

#[derive(Debug)]
struct RectRenderer{
	pipeline:wgpu::RenderPipeline,
	layout: wgpu::BindGroupLayout,
	snapshots:Vec<Rect>,
	window_bind_group:wgpu::BindGroup,
	bind_groups:Vec<wgpu::BindGroup>,
	buffers:Vec<wgpu::Buffer>,
	// the number of frames since the buffer was last used
	// last_used:u8 remove it when it reaches the max
}

impl RectRenderer {
	pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat) -> Self{
		// TODO create builders to reduce the boilerplate
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Rect Shader Module"),
			source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/rect.wgsl").into()),
		});

		let window_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Global window bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
	
		let rect_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("Rect layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 2,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				},
			],
		});

		// TODO replace with builder
		let vertex_buffer_layout = wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute { // Position
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2,
				},
				wgpu::VertexAttribute { // Color
					offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x4, 
				},
				wgpu::VertexAttribute { // UV
					offset: size_of::<[f32; 6]>() as wgpu::BufferAddress,
					shader_location: 2,
					format: wgpu::VertexFormat::Float32x2,
				},
			],
		};

        let window_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Window buffer"),
            usage:wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents:bytemuck::cast_slice(&[500.0,500.0]),
        });

		let window_bind_group = device.create_bind_group(
			&wgpu::BindGroupDescriptor {
				label: Some("Window bind group"),
				entries: &[
					wgpu::BindGroupEntry{
						binding:0,
						resource:window_buffer.as_entire_binding()
					}
				],
				layout:&window_layout,
			}
		);
	
		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Rect Pipeline Layout"),
			bind_group_layouts: &[&window_layout, &rect_layout],
			push_constant_ranges: &[],
		});
	
		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Rect Render Pipeline"),
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				compilation_options: Default::default(),
				buffers: &[vertex_buffer_layout],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				compilation_options: Default::default(),
				targets: &[Some(wgpu::ColorTargetState {
					format,
					blend: Some(wgpu::BlendState::ALPHA_BLENDING),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				unclipped_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			depth_stencil: None,
			multiview: None,
			cache: None,
		});

		Self{
			layout:rect_layout,
			pipeline,
			window_bind_group,
			snapshots:vec![],
			bind_groups:vec![],
			buffers:vec![],
		}
	}

	pub fn draw(
		&mut self,
		rect:&Rect,
		device:&wgpu::Device,
		resources:&mut ResourcePool,
		pass:&mut wgpu::RenderPass,
	){
		let vertices = Vertex::quad(rect.size, rect.position, rect.color);
	
		let vertex_buffer = BufferBuilder::new()
			.label("Rect vertex buffer")
			.vertex()
			.init(&vertices)
			.build(device);
	
		let size_buffer = resources.add_uniform_init(
			"Rect Size Buffer",
			bytemuck::cast_slice(&[rect.size.width, rect.size.height]),
			device,
		);
	
		let position_buffer = resources.add_uniform_init(
			"Rect Position Buffer",
			bytemuck::cast_slice(&[rect.position.x, rect.position.y]),
			device,
		);
	
		let radius_buffer = resources.add_uniform_init(
			"Rect Corner Radius Buffer",
			bytemuck::cast_slice(&[12.0]),
			device,
		);
	
		let bind_group_index = resources.add_bind_group(
			"Rect Bind Group",
			&self.layout,
			device,
			&[radius_buffer, size_buffer, position_buffer],
			&[],
			&[],
		).unwrap();

		let bind_group = resources.bind_group(bind_group_index).unwrap();

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.window_bind_group, &[]);
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
