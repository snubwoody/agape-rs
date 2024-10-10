use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{self, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5,], color: [1.0, 0.0, 0.0,1.0] },
    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0,1.0] },
    Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0,1.0] },
];

#[tokio::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

	let mut state = State::new(&window).await;

    event_loop.run(move |event, control_flow| {
		match event {
        Event::WindowEvent {ref event,window_id,} => match event {
            WindowEvent::CloseRequested => control_flow.exit(),
			WindowEvent::Resized(size) => state.resize(*size),
			WindowEvent::RedrawRequested => state.render().unwrap(),
            _ => {}
        },
        _ => {}
	    }
	}).unwrap();
}

struct State<'a>{
	surface: wgpu::Surface<'a>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
	render_pipeline: wgpu::RenderPipeline,
	vertex_buffer:wgpu::Buffer
}

impl<'a> State<'a> {
	async fn new(window:&'a Window) -> Self{
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
			backends: wgpu::Backends::PRIMARY,
			..Default::default()
		});

		// Create the surface to draw on
		let surface = instance.create_surface(window).unwrap();

		let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
			power_preference: Default::default(),
			compatible_surface: Some(&surface),
			force_fallback_adapter:false
		}).await.unwrap();

		let (device,queue) = adapter.request_device(&wgpu::DeviceDescriptor{
			required_features: wgpu::Features::empty(),
			required_limits:wgpu::Limits::default(),
			label: None,
			memory_hints: Default::default()
		}, None).await.unwrap();

		let surface_caps = surface.get_capabilities(&adapter);

		let surface_format = surface_caps
			.formats
			.iter()
			.find(|f|f.is_srgb())
			.copied()
			.unwrap_or(surface_caps.formats[0]);

		let config = wgpu::SurfaceConfiguration{
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode:surface_caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2
		};

		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
			label: Some("Shader module"), 
			source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into())
		});

		let render_pipeline_layout = 
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
				label: Some("Render pipeline layout"), 
				bind_group_layouts: &[], 
				push_constant_ranges: &[] 
			});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
			label: Some("Render pipeline"), 
			layout: Some(&render_pipeline_layout), 
			vertex: wgpu::VertexState { 
				module: &shader, 
				entry_point: "vs_main", 
				compilation_options: Default::default(), 
				buffers: &[Vertex::decription()] 
			}, 
			fragment: Some(wgpu::FragmentState{
				module:&shader,
				entry_point:"fs_main",
				compilation_options: Default::default(),
				targets:&[Some(wgpu::ColorTargetState { 
					format: config.format, 
					blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
					write_mask: wgpu::ColorWrites::ALL 
				})]
			}), 
			primitive: wgpu::PrimitiveState { 
				topology: wgpu::PrimitiveTopology::TriangleList, 
				strip_index_format: None, 
				front_face: wgpu::FrontFace::Ccw, 
				cull_mode: Some(wgpu::Face::Back), 
				unclipped_depth: false, 
				polygon_mode: wgpu::PolygonMode::Fill, 
				conservative: false 
			}, 
			multisample: wgpu::MultisampleState { 
				count: 1, 
				mask: !0, 
				alpha_to_coverage_enabled: false, 
			}, 
			multiview: None, 
			cache: None, 
			depth_stencil: None, 
		});

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(VERTICES),
			usage: wgpu::BufferUsages::VERTEX,
		});

		Self{
			surface,
			device,
			queue,
			config,
			size,
			window,
			render_pipeline,
			vertex_buffer
		}
	}

	fn resize(&mut self, new_size:winit::dpi::PhysicalSize<u32>){
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	fn update(&mut self){
		todo!()
	}

	fn render(&mut self) -> Result<(),wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
			label:Some("Render encoder")
		});

		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color {
						r: 0.1,
						g: 0.2,
						b: 0.3,
						a: 1.0,
					}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});

		render_pass.set_pipeline(&self.render_pipeline);
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.draw(0..3, 0..1);

		// Drop the render pass because it borrows encoder
		// mutably
		drop(render_pass);
	
		// submit will accept anything that implements IntoIter
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

#[repr(C)]
#[derive(Debug,Clone,Copy,bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex{
	position:[f32;2],
	color: [f32;4]	
}

impl Vertex {
	fn decription() -> wgpu::VertexBufferLayout<'static> {
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
