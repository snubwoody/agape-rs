use wgpu::{
	util::DeviceExt, ColorTargetState, Device, FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, VertexAttribute, VertexBufferLayout, VertexState
};
use crate::vertex::Vertex;
use helium_core::size::Size;

/// Holds the render pipeline
#[derive(Debug)]
pub struct RectRenderer{
	pub render_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
	pub bounds_layout: wgpu::BindGroupLayout,
    pub window_buffer: wgpu::Buffer,
}

impl RectRenderer {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
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
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
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

		// TODO PLEASE create uniform struct

		let bounds_layout = device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor{
				label:Some("Rect bounds layout"),
				entries:&[
					wgpu::BindGroupLayoutEntry{
						binding:0,
						visibility:wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count:None
					},
					wgpu::BindGroupLayoutEntry{
						binding:1,
						visibility:wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count:None
					},
					wgpu::BindGroupLayoutEntry{
						binding:2,
						visibility:wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count:None
					}
				],
			}
		);

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
					bind_group_layouts: &[&window_bind_group_layout,&bounds_layout],
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

		
		Self { 
			render_pipeline, 
			window_bind_group, 
			bounds_layout,
			window_buffer 
		}
	}
}

/// Controls the rendering of text to the screen
#[derive(Debug)]
pub struct TextRenderer{
	pub render_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
    pub window_buffer: wgpu::Buffer,
	pub texture_bind_group_layout: wgpu::BindGroupLayout
}

impl TextRenderer {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		let (render_pipeline,window_buffer,window_bind_group,texture_bind_group_layout) = 
			TextRenderer::create_pipeline(device, config, size);
		
		Self { 
			render_pipeline, 
			window_bind_group, 
			window_buffer,
			texture_bind_group_layout
		}
	}

	fn create_pipeline(
		device:&wgpu::Device,
		config:&wgpu::SurfaceConfiguration,
		size:&Size
	) -> (wgpu::RenderPipeline,wgpu::Buffer,wgpu::BindGroup,wgpu::BindGroupLayout) {
		// Compile the shader
		let shader = device.create_shader_module(
			ShaderModuleDescriptor{
				label: Some("Text Shader Module"),
				source: ShaderSource::Wgsl(include_str!("../shaders/text.wgsl").into())
			}
		);

		let window_uniform = UniformBuilder::new().contents(vec![size.width,size.height]).build(device);

		let texture_bind_group_layout = device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor { 
				label: Some("Text bind group layout"), 
				entries: &[
					// For the texture
					wgpu::BindGroupLayoutEntry{
						binding:0,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Texture { 
							sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
							view_dimension: wgpu::TextureViewDimension::D2, 
							multisampled: false
						},
						count:None
					},
					wgpu::BindGroupLayoutEntry{
						binding:1,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
						count:None
					}
				]
			}
		);

		let vertex_buffer_layout = VertexBufferLayout { 
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
					bind_group_layouts: &[&window_uniform.layout,&texture_bind_group_layout],
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
					buffers: &[vertex_buffer_layout]
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

		(render_pipeline,window_uniform.buffer,window_uniform.bind_group,texture_bind_group_layout)
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


pub struct UniformBuilder<T>{
	label:Option<String>,
	visibility:wgpu::ShaderStages,
	contents:Vec<T>
}

impl<T:bytemuck::Pod> UniformBuilder<T> {
	pub fn new() -> Self {
		Self { 
			label: None, 
			visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, 
			contents: vec![] 
		}
	}

	pub fn label(mut self,label:&str) -> Self{
		self.label = Some(label.into());
		self
	}

	pub fn visibility(mut self,visibility:wgpu::ShaderStages) -> Self{
		self.visibility = visibility;
		self
	}

	pub fn contents(mut self,contents:Vec<T>) -> Self{
		self.contents = contents;
		self
	}

	/// Build a uniform buffer
	pub fn build(self,device:&wgpu::Device) -> Uniform{
		let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: self.label.clone().map(|label|format!("{} buffer",label)).as_deref(),
			contents: bytemuck::cast_slice(&self.contents),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});
		
		let layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: self.label.clone().map(|label|format!("{} bind group layout",label)).as_deref(),
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: self.visibility,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});
		
		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: self.label.map(|label|format!("{} bind group",label)).as_deref(),
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
		});

		Uniform { buffer,layout,bind_group }
	}
}

/// A uniform buffer
pub struct Uniform{
	buffer:wgpu::Buffer,
	layout:wgpu::BindGroupLayout,
	bind_group:wgpu::BindGroup
}

impl Uniform {
	pub fn buffer(&self) -> &wgpu::Buffer{
		&self.buffer
	}

	pub fn layout(&self) -> &wgpu::BindGroupLayout{
		&self.layout
	}
	pub fn bind_group(&self) -> &wgpu::BindGroup{
		&self.bind_group
	}
}