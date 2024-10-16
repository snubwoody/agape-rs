pub mod view;
pub mod events;
use wgpu::{util::DeviceExt, BindGroupDescriptor, BindGroupLayoutDescriptor};
use winit::{
	dpi::PhysicalSize, event::WindowEvent, event_loop::{
		ControlFlow, 
		EventLoop
	}, window::{Window, WindowBuilder}
};
use view::View;
use crate::utils::Size;
use async_std::task;



/// This is a singular isolated program. Most projects
/// will only contain one app
pub struct App{
	event_loop:EventLoop<()>,
	window:Window,
	views:Vec<View>,
	index:usize,
}

impl App{
	pub fn new() -> Self {
		let event_loop = EventLoop::new().unwrap();

		// Set the control flow to redraw every frame whether
		// there are events to process or not
		event_loop.set_control_flow(ControlFlow::Poll);
		
		let window = WindowBuilder::new().build(&event_loop).unwrap();

		Self { 
			event_loop,
			window,
			views:vec![],
			index:0,
		}
	}

	pub fn add_view(mut self,view:View) -> Self{
		self.views.push(view);
		self
	}

	pub fn run(mut self){
		let mut state = task::block_on(AppState::new(&self.window));

		self.event_loop.run(move| event,window_target|{
			match event {
				winit::event::Event::WindowEvent{event,..} => match event {
					WindowEvent::CloseRequested => window_target.exit(),
					WindowEvent::RedrawRequested => self.views[self.index].render(&state),
					WindowEvent::Resized(size) => state.resize(size),
					_ => {}
				}, 
				_ => {}
			}
	
		}).expect("Event loop error occured");
	}
}

pub struct AppState<'a>{
	pub surface: wgpu::Surface<'a>,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub context:RenderContext,
	pub size: Size
}

impl<'a> AppState<'a> {
	pub async fn new(window:&'a Window) -> Self{
		let size = window.inner_size().into();

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
			.find(|s|s.is_srgb())
			.copied()
			.unwrap_or(surface_caps.formats[0]);

		// The surface configuration
		let config = wgpu::SurfaceConfiguration{
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: window.inner_size().width,
			height: window.inner_size().height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode:surface_caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2
		};

		// Configure the surface for presentation
		surface.configure(&device, &config);

		let context = RenderContext::new(&device, &config,&size);

		Self{
			surface,
			device,
			queue,
			context,
			size
		}	
	}

	pub fn resize(&mut self,size:PhysicalSize<u32>) {
		self.size = size.into();
		self.queue.write_buffer(
			&self.context.window_buffer, 
			0, 
			bytemuck::cast_slice(&[self.size.width,self.size.height])
		);
	}
}

/// Holds the compiled shaders
#[derive(Debug)]
pub struct RenderContext{
	pub rect_pipeline: wgpu::RenderPipeline,
	pub text_pipeline: wgpu::RenderPipeline,
	pub image_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
	pub window_buffer: wgpu::Buffer
}

impl RenderContext {
	pub fn new(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration,size:&Size) -> Self{
		let (rect_pipeline,window_bind_group,window_buffer) = 
			RenderContext::create_rect_pipeline(device, config,size);

			Self{
			rect_pipeline,
			window_bind_group,
			window_buffer,
			text_pipeline: RenderContext::create_text_pipeline(device, config),
			image_pipeline: RenderContext::create_image_pipeline(device, config)
		}
	}

	fn create_rect_pipeline(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration,size:&Size) 
	-> (wgpu::RenderPipeline,wgpu::BindGroup,wgpu::Buffer) {
		// Compiled shader
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
			label: Some("Shader module"), 
			source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect.wgsl").into())
		});

		let window_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Window buffer"),
			// Pass the window size as a uniform
			contents:bytemuck::cast_slice(&[size.width,size.height]), 
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
		});

		// The layout for the window uniform 
		let window_bind_group_layout = device.create_bind_group_layout(
			&BindGroupLayoutDescriptor{
				label: Some("Window binding layout"),
				entries:&[
					wgpu::BindGroupLayoutEntry{
						binding:0,
						visibility:wgpu::ShaderStages::VERTEX,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count: None
					}
				],
			}
		);

		let window_bind_group = device.create_bind_group(
			&BindGroupDescriptor{
				label: Some("Window Bind Group"),
				layout: &window_bind_group_layout,
				entries:&[
					wgpu::BindGroupEntry{
						binding:0,
						resource: window_buffer.as_entire_binding()
					}
				]
			}
		);
		
		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&wgpu::PipelineLayoutDescriptor { 
					label: Some("Render pipeline layout"), 
					bind_group_layouts: &[&window_bind_group_layout], 
					push_constant_ranges: &[] 
				}
			);

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
			label: Some("Render pipeline"), 
			layout: Some(&render_pipeline_layout), 
			vertex: wgpu::VertexState { 
				module: &shader, 
				entry_point: "vs_main", 
				compilation_options: Default::default(), 
				buffers: &[crate::vertex::Vertex::decription()] // Move this to this function
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
				cull_mode: None, 
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

		return (render_pipeline,window_bind_group,window_buffer)
	}

	fn create_text_pipeline(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
		// TODO replace this with the actual text shader
		// Compiled shader
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
			label: Some("Shader module"), 
			source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into())
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
				buffers: &[crate::vertex::Vertex::decription()] 
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
				cull_mode: None, 
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
			source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into())
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
				buffers: &[crate::vertex::Vertex::decription()]
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
				cull_mode: None, 
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
