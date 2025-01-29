use super::GlobalResources;
use crate::{
    builders::{
        BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder, TextureBuilder,
        VertexBufferLayoutBuilder,
    },
    primitives::Image,
    vertex::Vertex,
};
use helium_core::{
    color::{TRANSPARENT, WHITE},
    Size,
};
use std::rc::Rc;
use wgpu::Extent3d;

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    global: Rc<GlobalResources>,
}

impl ImagePipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        global: Rc<GlobalResources>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Image Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/image.wgsl").into()),
        });

        let layout = BindGroupLayoutBuilder::new()
            .label("Image bind group layout")
            .texture(
                wgpu::ShaderStages::FRAGMENT,
                wgpu::TextureSampleType::Float { filterable: true },
                wgpu::TextureViewDimension::D2,
                false,
            )
            .sampler(
                wgpu::ShaderStages::FRAGMENT,
                wgpu::SamplerBindingType::Filtering,
            )
            .build(device);

        let vertex_buffer_layout = VertexBufferLayoutBuilder::new()
            .array_stride(size_of::<Vertex>() as u64)
            .attribute(0, wgpu::VertexFormat::Float32x2)
            .attribute(size_of::<[f32; 2]>() as u64, wgpu::VertexFormat::Float32x4)
            .attribute(size_of::<[f32; 6]>() as u64, wgpu::VertexFormat::Float32x2)
            .build();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Icon Pipeline Layout"),
            bind_group_layouts: &[global.window_layout(), &layout],
            push_constant_ranges: &[],
        });

        // TODO create a builder for this
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Icon Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[vertex_buffer_layout],
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

        Self {
            pipeline,
            layout,
            global,
        }
    }

    pub fn draw(
        &mut self,
        image: &Image,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass,
    ) {
        // Rasterize the text
        // Should hopefully replace this library eventually with something glyph based
        let image_data = image.image.to_rgba8();

        let size = Size {
            width: image_data.dimensions().0 as f32,
            height: image_data.dimensions().1 as f32,
        };

        let vertices = Vertex::quad(size, image.position, TRANSPARENT);

        let vertex_buffer = BufferBuilder::new()
            .label("Image vertex buffer")
            .vertex()
            .init(&vertices)
            .build(device);

        let texture = TextureBuilder::new()
            .label("Image texture")
            .size(size)
            .dimension(wgpu::TextureDimension::D2)
            .format(wgpu::TextureFormat::Rgba8UnormSrgb)
            .usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(device);

        let texture_view = texture.create_view(&Default::default());
        let sampler = device.create_sampler(&Default::default());

        let bind_group = BindGroupBuilder::new()
            .label("Image bind group")
            .texture_view(&texture_view)
            .sampler(&sampler)
            .build(&self.layout, device);

        let size = Extent3d {
            width: size.width as u32,
            height: size.height as u32,
            depth_or_array_layers: 1,
        };

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width as u32),
                rows_per_image: Some(size.height as u32),
            },
            size,
        );

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, self.global.window_bind_group(), &[]);
        pass.set_bind_group(1, &bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..vertices.len() as u32, 0..1);
    }
}
