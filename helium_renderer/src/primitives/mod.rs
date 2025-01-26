use crate::Vertex;

pub struct RectShader{
	/// The index of bind group in the [`ResourcePool`].
	window_layout:wgpu::BindGroupLayout,
	layout:wgpu::BindGroupLayout,
	pipeline:wgpu::RenderPipeline
}

impl RectShader{
	pub fn new(
		device:&wgpu::Device,
		format:wgpu::TextureFormat
	) -> Result<Self,crate::Error>{
		
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Rect Shader Module"),
			source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect.wgsl").into()),
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
		let buffer_layout = wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 6]>() as wgpu::BufferAddress,
					shader_location: 2,
					format: wgpu::VertexFormat::Float32x2,
				},
			],
		};
	
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
				buffers: &[buffer_layout],
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

		Ok(
			Self { 
				pipeline,
				window_layout, 
				layout: rect_layout
			}
		)
	}

	pub fn layout(&self) -> &wgpu::BindGroupLayout {
		&self.layout
	}

	pub fn pipeline(&self) -> &wgpu::RenderPipeline {
		&self.pipeline
	}

	/// Get the index of the `wgpu::BindGroup` in the [`ResourcePool`]
	pub fn window_layout(&self) -> &wgpu::BindGroupLayout{
		&self.window_layout
	}
	
}
