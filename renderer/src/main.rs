use wgpu::{util::DeviceExt, RenderPipeline, SurfaceConfiguration, TextureFormat};
mod vertex;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{self, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
use crate::vertex::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5,], color: [1.0, 0.0, 0.0,1.0] },
    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0,1.0] },
    Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0,1.0] },
];

#[tokio::main]
async fn main() {
	// FIXME wgpu expects colours in a linear colour space, 
	// not sRGB, so perform colour correction.
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
	context: RenderContext,
	vertex_buffer:wgpu::Buffer
}

impl<'a> State<'a> {
	async fn new(window:&'a Window) -> Self{
		let size = window.inner_size();

		// Handle to wpgu for creating a surface and an adapter
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
			backends: wgpu::Backends::PRIMARY,
			..Default::default()
		});

		// Create the surface to draw on
		let surface = instance.create_surface(window).unwrap();

		// Handle to the graphics card
		let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
			power_preference: Default::default(),
			compatible_surface: Some(&surface),
			force_fallback_adapter:false
		}).await.unwrap();

		// The device is an open connection to the graphics
		// card and the queue is a command buffer
		let (device,queue) = adapter.request_device(&wgpu::DeviceDescriptor{
			label: Some("Device/Queue"),
			required_features: wgpu::Features::empty(),
			..Default::default()
		}, None).await.unwrap();

		let surface_caps = surface.get_capabilities(&adapter);

		// Get an sRGB texture format
		let surface_format = 
			surface_caps
			.formats
			.iter()
			.find(|f|f.is_srgb())
			.copied()
			.unwrap_or(surface_caps.formats[0]);

		// The surface configuration
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

		// Create a new render context
		let context = RenderContext::new(&device, &config);

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
			context,
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

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&self.context.rect_pipeline);
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

		render_pass.draw(0..VERTICES.len() as u32, 0..1);

		// Drop the render pass because it borrows encoder
		// mutably
		drop(render_pass);
	
		// submit will accept anything that implements IntoIter
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

/// Holds the compiled shaders
#[derive(Debug)]
pub struct RenderContext{
	rect_pipeline: wgpu::RenderPipeline,
	text_pipeline: wgpu::RenderPipeline,
	image_pipeline: wgpu::RenderPipeline
}

impl RenderContext {
	pub fn new(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> Self{
		Self{
			rect_pipeline:RenderContext::create_rect_pipeline(device, config),
			text_pipeline: RenderContext::create_text_pipeline(device, config),
			image_pipeline: RenderContext::create_image_pipeline(device, config)
		}
	}

	fn create_rect_pipeline(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
		// TODO replace this with the actual text shader
		// Compiled shader
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
					blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending 
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

		render_pipeline
	}

	fn create_text_pipeline(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
		// TODO replace this with the actual text shader
		// Compiled shader
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
					blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending 
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

		render_pipeline
	}

	fn create_image_pipeline(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
		// TODO replace this with the actual text shader
		// Compiled shader
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
					blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending 
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

		render_pipeline
	}
}

/* // TODO try fitting the window and display in the render context
/// Contains the compiled shader programs
#[derive(Debug)]
pub struct RenderContext{
	pub surface_program:Program,
	pub text_program:Program,
	pub image_program:Program
}

impl RenderContext {
	// TODO change this to use the from source method of the Program struct
	pub fn new(
		surface_program:Program,
		text_program:Program,
		image_program:Program
	) -> Self {
		Self{ 
			surface_program, 
			text_program,
			image_program
		}
	}
} */


