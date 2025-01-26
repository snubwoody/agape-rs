use crate::{
    builders::{BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder},
    rect::Rect,
    vertex::Vertex,
};

pub struct RectPipeline {
    rect_layout: wgpu::BindGroupLayout,
}

impl RectPipeline {
    pub fn new(device: &wgpu::Device,format:wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rect Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect.wgsl").into()),
        });

        let window_layout = BindGroupLayoutBuilder::new()
            .label("Global window bind group layout")
            .uniform(wgpu::ShaderStages::VERTEX_FRAGMENT)
            .build(device);

        let rect_layout = BindGroupLayoutBuilder::new()
            .label("Rect bind group layout")
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .build(device);

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

        todo!()
    }

    pub fn draw(
		&mut self,
		rect: &Rect,
		device: &wgpu::Device, 
		pass: &mut wgpu::RenderPass, 
	) {

        let vertices = Vertex::quad(rect.size, rect.position, rect.color);

        let vertex_buffer = BufferBuilder::new()
            .label("Rect vertex buffer")
            .vertex()
            .init(&vertices)
            .build(device);

        let size = BufferBuilder::new()
            .label("Rect size buffer")
            .uniform()
            .copy_dst()
            .init(&[rect.size])
            .build(device);

        let position = BufferBuilder::new()
            .label("Rect position buffer")
            .uniform()
            .copy_dst()
            .init(&[rect.position])
            .build(device);

        let corner_radius = BufferBuilder::new()
            .label("Rect corner radius buffer")
            .uniform()
            .copy_dst()
            .init(&[12.0])
            .build(device);

        // let rect_bind_group = BindGroupBuilder::new()
        //     .label("Rect bind group")
        //     .buffer(&corner_radius)
        //     .buffer(&size)
        //     .buffer(&position)
        //     .build(self.shader.layout(), device);

        // pass.set_pipeline(self.shader.pipeline());
        // pass.set_bind_group(0, &self.window_bind_group, &[]);
        // pass.set_bind_group(1, &rect_bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..vertices.len() as u32, 0..1);
    }
}
