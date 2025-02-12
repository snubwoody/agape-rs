use helium_core::Size;
use std::rc::Rc;

use super::GlobalResources;
use crate::{
    builders::{
        BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder, VertexBufferLayoutBuilder,
    },
    primitives::{Circle, Rect},
    vertex::Vertex,
};

pub struct CirclePipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    global: Rc<GlobalResources>,
}

impl CirclePipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        global: Rc<GlobalResources>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Circle Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/circle.wgsl").into()),
        });

        let layout = BindGroupLayoutBuilder::new()
            .label("Circle bind group layout")
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .build(device);

        let vertex_buffer_layout = VertexBufferLayoutBuilder::new()
            .array_stride(size_of::<Vertex>() as u64)
            .attribute(0, wgpu::VertexFormat::Float32x2)
            .attribute(size_of::<[f32; 2]>() as u64, wgpu::VertexFormat::Float32x4)
            .attribute(size_of::<[f32; 6]>() as u64, wgpu::VertexFormat::Float32x2)
            .build();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Circle Pipeline Layout"),
            bind_group_layouts: &[global.window_layout(), &layout],
            push_constant_ranges: &[],
        });

        // TODO create a builder for this
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Circle Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
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

        Self {
            pipeline,
            layout,
            global,
        }
    }

    pub fn draw(&mut self, circle: &Circle, device: &wgpu::Device, pass: &mut wgpu::RenderPass) {
        let vertices = Vertex::quad(Size::unit(circle.diameter), circle.position, circle.color.clone());

        let vertex_buffer = BufferBuilder::new()
            .label("Circle vertex buffer")
            .vertex()
            .init(&vertices)
            .build(device);

        let diameter = BufferBuilder::new()
            .label("Circle diameter buffer")
            .uniform()
            .copy_dst()
            .init(&[circle.diameter])
            .build(device);

        let position = BufferBuilder::new()
            .label("Rect position buffer")
            .uniform()
            .copy_dst()
            .init(&[circle.position])
            .build(device);

        let rect_bind_group = BindGroupBuilder::new()
            .label("Rect bind group")
            .buffer(&diameter)
            .buffer(&position)
            .build(&self.layout, device);

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, self.global.window_bind_group(), &[]);
        pass.set_bind_group(1, &rect_bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..vertices.len() as u32, 0..1);
    }
}
