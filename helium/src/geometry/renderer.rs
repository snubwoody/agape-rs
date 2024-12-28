use std::str;

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor, ColorTargetState, FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, VertexAttribute, VertexBufferLayout, VertexState
};
use crate::geometry::vertex::Vertex;
use helium_core::size::Size;

use super::{uniform::{Uniform, UniformBuilder}, vertex::VertexBufferLayoutBuilder};

// TODO could maybe move the buffers into herer
// TODO pls refactor this long and ugly code, there's a lot of reused code;
/// Holds the render pipeline
#[derive(Debug)]
pub struct RectRenderContext{
	pub render_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
	pub bounds_layout: wgpu::BindGroupLayout,
    pub window_buffer: wgpu::Buffer,
}

impl RectRenderContext {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		// Compile the shaders
		let shader = device.create_shader_module(
			ShaderModuleDescriptor{
				label: Some("Rect Shader Module"),
				source: ShaderSource::Wgsl(include_str!("../../shaders/rect.wgsl").into())
			}
		);

		let window_uniform = 
			UniformBuilder::new()
			.label("Window")
			.contents(&[size.width,size.height])
			.build(device);

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
					bind_group_layouts: &[window_uniform.layout(),&bounds_layout],
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
						blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
			window_bind_group:window_uniform.bind_group, 
			bounds_layout,
			window_buffer:window_uniform.buffer
		}
	}
}

/// Holds the render pipeline
pub struct CircleRenderContext{
	pub render_pipeline: wgpu::RenderPipeline,
	pub bounds_layout: wgpu::BindGroupLayout,
	pub window_uniform:Uniform,
	pub bounds_bind_group:wgpu::BindGroup,
	pub position_buffer:wgpu::Buffer,
	pub diameter_buffer:wgpu::Buffer
}

impl CircleRenderContext {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		// Compile the shader
		let shader = device.create_shader_module(
			ShaderModuleDescriptor{
				label: Some("Circle Shader Module"),
				source: ShaderSource::Wgsl(include_str!("../../shaders/circle.wgsl").into())
			}
		);

		let window_uniform = 
			UniformBuilder::new()
			.label("Window")
			.contents(&[size.width,size.height])
			.build(device);

		let diameter_buffer = device.create_buffer_init(
			&BufferInitDescriptor{
				label:Some("Size buffer"),
				contents: bytemuck::cast_slice(&[0.0,0.0]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}
		);

		let position_buffer = device.create_buffer_init(
			&BufferInitDescriptor{
				label:Some("Position buffer"),
				contents: bytemuck::cast_slice(&[0.0,0.0]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}
		);

		let bounds_layout = device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor{
				label:Some("Circle bounds layout"),
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
				],
			}
		);

		
		let bounds_bind_group = device.create_bind_group(
			&BindGroupDescriptor{
				label:Some("Cirlce bounds bind group"),
				layout:&bounds_layout,
				entries:&[
					wgpu::BindGroupEntry{
						binding:0,
						resource:diameter_buffer.as_entire_binding()
					},
					wgpu::BindGroupEntry{
						binding:1,
						resource:position_buffer.as_entire_binding()
					}
				]
			}
		);

		let buffer_layout = 
			VertexBufferLayoutBuilder::new()
			.add_attribute(0, wgpu::VertexFormat::Float32x2)
			.add_attribute(size_of::<[f32;2]>(), wgpu::VertexFormat::Float32x4)
			.add_attribute(size_of::<[f32;6]>(), wgpu::VertexFormat::Float32x2)
			.build();

		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some("Circle Pipeline Layout"),
					bind_group_layouts: &[window_uniform.layout(),&bounds_layout],
					push_constant_ranges: &[]
				}
			);

		let render_pipeline = 
			RenderPipelineBuilder::new("Circle",&shader)
			.add_bind_group_layout(&window_uniform.layout)
			.add_bind_group_layout(&bounds_layout)
			.layout(&render_pipeline_layout)
			.add_buffer(buffer_layout)
			.build(device, config);

		Self { 
			render_pipeline, 
			window_uniform,
			bounds_layout,
			bounds_bind_group,
			position_buffer,
			diameter_buffer
		}
	}
}

// TODO test this
/// Controls the rendering of text to the screen
#[derive(Debug)]
pub struct TextRenderContext{
	pub render_pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
    pub window_buffer: wgpu::Buffer,
	pub texture_bind_group_layout: wgpu::BindGroupLayout
}

impl TextRenderContext {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		let (render_pipeline,window_buffer,window_bind_group,texture_bind_group_layout) = 
			TextRenderContext::create_pipeline(device, config, size);
		
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
				source: ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into())
			}
		);

		let window_uniform = 
			UniformBuilder::new()
			.label("Window")
			.contents(&[size.width,size.height])
			.build(device);

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

		let vertex_buffer_layout = 
			VertexBufferLayoutBuilder::new()
			.add_attribute(0, wgpu::VertexFormat::Float32x2)
			.add_attribute(size_of::<[f32;2]>(), wgpu::VertexFormat::Float32x4)
			.add_attribute(size_of::<[f32;6]>(), wgpu::VertexFormat::Float32x2)
			.build();

		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some("Text Pipeline Layout"),
					bind_group_layouts: &[&window_uniform.layout,&texture_bind_group_layout],
					push_constant_ranges: &[]
				}
			);

		let render_pipeline = 
			RenderPipelineBuilder::new("Text",&shader)
			.layout(&render_pipeline_layout)
			.add_bind_group_layout(&window_uniform.layout)
			.add_bind_group_layout(&texture_bind_group_layout)
			.add_buffer(vertex_buffer_layout)
			.build(device, config);

		(
			render_pipeline,
			window_uniform.buffer,
			window_uniform.bind_group,
			texture_bind_group_layout
		)
	}
}


pub struct RenderPipelineBuilder<'a>{
	label:String,
	shader:&'a wgpu::ShaderModule,
	vertex_entry_point:String,
	fragment_entry_point:String,
	layout:Option<&'a wgpu::PipelineLayout>,
	bind_group_layouts:Vec<&'a wgpu::BindGroupLayout>,
	buffers:Vec<VertexBufferLayout<'a>>
}

impl<'a> RenderPipelineBuilder<'a> {
	pub fn new(label:&str,shader:&'a wgpu::ShaderModule) -> Self{
		let vertex_entry_point = String::from("vs_main");
		let fragment_entry_point = String::from("fs_main");
		
		Self{
			label:label.to_owned(),
			shader,
			vertex_entry_point,
			fragment_entry_point,
			buffers:vec![],
			bind_group_layouts:vec![],
			layout:None
		}
	}


	pub fn vertex_entry_point(mut self,entry_point:&str) -> Self {
		self.vertex_entry_point = entry_point.to_owned();
		self
	}

	pub fn fragment_entry_point(mut self,entry_point:&str) -> Self {
		self.fragment_entry_point = entry_point.to_owned();
		self
	}

	pub fn add_bind_group_layout(mut self,layout:&'a wgpu::BindGroupLayout) -> Self{
		self.bind_group_layouts.push(layout);
		self
	}

	pub fn layout(mut self,layout:&'a wgpu::PipelineLayout) -> Self{
		self.layout = Some(layout);
		self
	}

	pub fn add_buffer(mut self,buffer:wgpu::VertexBufferLayout<'a>) -> Self{
		self.buffers.push(buffer);
		self
	}

	pub fn build(self,device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline{
		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some(format!("{} Pipeline Layout",self.label).as_str()),
					bind_group_layouts: self.bind_group_layouts.as_slice(),
					push_constant_ranges: &[]
				}
			);

		device.create_render_pipeline(
			&RenderPipelineDescriptor { 
				label: Some(format!("{} Pipeline Layout",self.label).as_str()), 
				layout: Some(&render_pipeline_layout), 
				vertex: VertexState{
					module: &self.shader,
					entry_point: &self.vertex_entry_point,
					compilation_options: Default::default(),
					buffers: &self.buffers
				}, 
				fragment: Some(FragmentState{
					module: &self.shader,
					entry_point: &self.fragment_entry_point,
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
		)
	}
}

/// Contains the renderers
pub struct RenderContext {
	pub rect_renderer: RectRenderContext,
	pub text_renderer: TextRenderContext,
	pub circle_renderer: CircleRenderContext,
	pub window_uniform:Uniform
}

impl RenderContext {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, size: &Size) -> Self {
		let rect_renderer = RectRenderContext::new(device, config, size);
		let text_renderer = TextRenderContext::new(device, config, size);
		let circle_renderer = CircleRenderContext::new(device, config, size);

		let window_buffer = UniformBuilder::new()
			.label("Window uniform")
			.contents(&[size.width,size.height])
			.build(device);
		
        Self {
			rect_renderer,
			text_renderer,
			circle_renderer,
			window_uniform:window_buffer
        }
    }
}