use crate::{
    geometry::uniform::UniformBuilder,
    geometry::vertex::VertexBufferLayoutBuilder,
};
use helium_core::size::Size;
use wgpu::{
    PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource,
};

use super::RenderPipelineBuilder;

/// Holds the buffer and pipeline for rendering text to the screen
#[derive(Debug)]
pub struct TextPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub window_bind_group: wgpu::BindGroup,
    pub window_buffer: wgpu::Buffer,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl TextPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, size: &Size) -> Self {
        let (pipeline, window_buffer, window_bind_group, texture_bind_group_layout) =
            TextPipeline::create_pipeline(device, config, size);

        Self {
            pipeline,
            window_bind_group,
            window_buffer,
            texture_bind_group_layout,
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        size: &Size,
    ) -> (
        wgpu::RenderPipeline,
        wgpu::Buffer,
        wgpu::BindGroup,
        wgpu::BindGroupLayout,
    ) {
        // Compile the shader
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Text Shader Module"),
            source: ShaderSource::Wgsl(include_str!("../../../shaders/text.wgsl").into()),
        });

        let window_uniform = UniformBuilder::new()
            .label("Window")
            .contents(&[size.width, size.height])
            .build(device);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Text bind group layout"),
                entries: &[
                    // For the texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let vertex_buffer_layout = VertexBufferLayoutBuilder::new()
            .add_attribute(0, wgpu::VertexFormat::Float32x2)
            .add_attribute(size_of::<[f32; 2]>(), wgpu::VertexFormat::Float32x4)
            .add_attribute(size_of::<[f32; 6]>(), wgpu::VertexFormat::Float32x2)
            .build();

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&window_uniform.layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = RenderPipelineBuilder::new("Text", &shader)
            .layout(&pipeline_layout)
            .add_bind_group_layout(&window_uniform.layout)
            .add_bind_group_layout(&texture_bind_group_layout)
            .add_buffer(vertex_buffer_layout)
            .build(device, config);

        (
            pipeline,
            window_uniform.buffer,
            window_uniform.bind_group,
            texture_bind_group_layout,
        )
    }
}
