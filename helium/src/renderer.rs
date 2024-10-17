use wgpu::{
	util::DeviceExt, 
	ColorTargetState, 
	FragmentState, 
	MultisampleState, 
	PipelineLayoutDescriptor, 
	PrimitiveState, 
	RenderPipelineDescriptor, 
	ShaderModuleDescriptor, 
	ShaderSource, 
	VertexAttribute, 
	VertexBufferLayout, 
	VertexState
};
use crate::{utils::Size, vertex::Vertex};

/// Holds the render pipeline
#[derive(Debug)]
pub struct RectRenderer{
	pub render_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
    pub window_buffer: wgpu::Buffer,
}

impl RectRenderer {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		let (render_pipeline,window_buffer,window_bind_group) = 
			RectRenderer::create_pipeline(device, config, size);
		
		Self { 
			render_pipeline, 
			window_bind_group, 
			window_buffer 
		}
	}

	fn create_pipeline(
		device:&wgpu::Device,
		config:&wgpu::SurfaceConfiguration,
		size:&Size
	) -> (wgpu::RenderPipeline,wgpu::Buffer,wgpu::BindGroup) {
		// Compile the shader
		let shader = device.create_shader_module(
			ShaderModuleDescriptor{
				label: Some("Rect Shader Module"),
				source: ShaderSource::Wgsl(include_str!("../shaders/rect.wgsl").into())
			}
		);

		let window_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Window buffer"),
            // Pass the window size as a uniform
            contents: bytemuck::cast_slice(&[size.width, size.height]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // The layout for the window uniform
        let window_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Window binding layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let window_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Window Bind Group"),
            layout: &window_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: window_buffer.as_entire_binding(),
            }],
        });

		let buffer_layout = VertexBufferLayout { 
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
			step_mode: wgpu::VertexStepMode::Vertex, 
			attributes: &[
				VertexAttribute{
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2
				},
				VertexAttribute{
					offset: size_of::<[f32;2]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x4
				},
				VertexAttribute{
					offset: size_of::<[f32;6]>() as wgpu::BufferAddress,
					shader_location: 2,
					format: wgpu::VertexFormat::Float32x2 
				},
			]
		};

		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some("Rect Pipeline Layout"),
					bind_group_layouts: &[&window_bind_group_layout],
					push_constant_ranges: &[]
				}
			);

		let render_pipeline = device.create_render_pipeline(
			&RenderPipelineDescriptor { 
				label: Some("Rect Render Pipeline"), 
				layout: Some(&render_pipeline_layout), 
				vertex: VertexState{
					module: &shader,
					entry_point: "vs_main",
					compilation_options: Default::default(),
					buffers: &[buffer_layout]
				}, 
				fragment: Some(FragmentState{
					module: &shader,
					entry_point: "fs_main",
					compilation_options: Default::default(),
					targets: &[Some(ColorTargetState {
						format: config.format,
						blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending
						write_mask: wgpu::ColorWrites::ALL,
					})]
				}), 
				primitive: PrimitiveState{
					topology: wgpu::PrimitiveTopology::TriangleList,
                	strip_index_format: None,
                	front_face: wgpu::FrontFace::Ccw,
                	cull_mode: None,
                	unclipped_depth: false,
                	polygon_mode: wgpu::PolygonMode::Fill,
                	conservative: false,
				}, 
				multisample: MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				}, 
				depth_stencil: None, 
				multiview: None, 
				cache: None 
			}
		);

		(render_pipeline,window_buffer,window_bind_group)
	}
}

/// Controls the rendering of text to the screen
#[derive(Debug)]
pub struct TextRenderer{
	pub render_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
    pub window_buffer: wgpu::Buffer,
}

impl TextRenderer {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		let (render_pipeline,window_buffer,window_bind_group) = 
			RectRenderer::create_pipeline(device, config, size);
		
		Self { 
			render_pipeline, 
			window_bind_group, 
			window_buffer 
		}
	}

	fn create_pipeline(
		device:&wgpu::Device,
		config:&wgpu::SurfaceConfiguration,
		size:&Size
	) -> (wgpu::RenderPipeline,wgpu::Buffer,wgpu::BindGroup) {
		// Compile the shader
		let shader = device.create_shader_module(
			ShaderModuleDescriptor{
				label: Some("Text Shader Module"),
				source: ShaderSource::Wgsl(include_str!("../shaders/text.wgsl").into())
			}
		);

		let window_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Window buffer"),
            // Pass the window size as a uniform
            contents: bytemuck::cast_slice(&[size.width, size.height]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // The layout for the window uniform
        let window_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Window binding layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let window_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Window Bind Group"),
            layout: &window_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: window_buffer.as_entire_binding(),
            }],
        });

		let buffer_layout = VertexBufferLayout { 
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
			step_mode: wgpu::VertexStepMode::Vertex, 
			attributes: &[
				VertexAttribute{
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2
				},
				VertexAttribute{
					offset: size_of::<[f32;2]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x4
				},
				VertexAttribute{
					offset: size_of::<[f32;6]>() as wgpu::BufferAddress,
					shader_location: 2,
					format: wgpu::VertexFormat::Float32x2 
				},
			]
		};

		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some("Text Pipeline Layout"),
					bind_group_layouts: &[&window_bind_group_layout],
					push_constant_ranges: &[]
				}
			);

		let render_pipeline = device.create_render_pipeline(
			&RenderPipelineDescriptor { 
				label: Some("Text Render Pipeline"), 
				layout: Some(&render_pipeline_layout), 
				vertex: VertexState{
					module: &shader,
					entry_point: "vs_main",
					compilation_options: Default::default(),
					buffers: &[buffer_layout]
				}, 
				fragment: Some(FragmentState{
					module: &shader,
					entry_point: "fs_main",
					compilation_options: Default::default(),
					targets: &[Some(ColorTargetState {
						format: config.format,
						blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending
						write_mask: wgpu::ColorWrites::ALL,
					})]
				}), 
				primitive: PrimitiveState{
					topology: wgpu::PrimitiveTopology::TriangleList,
                	strip_index_format: None,
                	front_face: wgpu::FrontFace::Ccw,
                	cull_mode: None,
                	unclipped_depth: false,
                	polygon_mode: wgpu::PolygonMode::Fill,
                	conservative: false,
				}, 
				multisample: MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				}, 
				depth_stencil: None, 
				multiview: None, 
				cache: None 
			}
		);

		(render_pipeline,window_buffer,window_bind_group)
	}
}

/// Controls the rendering of images to the screen
#[derive(Debug)]
pub struct ImageRenderer{
	pub render_pipeline: wgpu::RenderPipeline
}

impl ImageRenderer {
	pub fn new(){

	}

	pub fn create_pipeline(){
		
	}
}