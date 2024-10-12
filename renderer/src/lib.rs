pub mod vertex;
use vertex::Vertex;

/// Holds the compiled shaders
#[derive(Debug)]
pub struct RenderContext{
	pub rect_pipeline: wgpu::RenderPipeline,
	pub text_pipeline: wgpu::RenderPipeline,
	pub image_pipeline: wgpu::RenderPipeline
}

impl RenderContext {
	pub fn new(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> Self{
		Self{
			rect_pipeline: RenderContext::create_rect_pipeline(device, config),
			text_pipeline: RenderContext::create_text_pipeline(device, config),
			image_pipeline: RenderContext::create_image_pipeline(device, config)
		}
	}

	fn create_rect_pipeline(device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
		// Compiled shader
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
			label: Some("Shader module"), 
			source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/rect.wgsl").into())
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
			source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/rect.wgsl").into())
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
			source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/rect.wgsl").into())
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



