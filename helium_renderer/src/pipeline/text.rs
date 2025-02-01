use super::GlobalResources;
use crate::{
    builders::{
        BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder, TextureBuilder,
        VertexBufferLayoutBuilder,
    },
    primitives::Text,
    vertex::Vertex,
};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, SwashCache, Weight};
use helium_core::Size;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::{io::Cursor, rc::Rc, time::Instant};
use wgpu::{
    hal::auxil::db::{self, imgtec},
    Extent3d,
};

// TODO replace text_to_png
/// The pipeline that handles text rendering.
/// It uses `cosmic_text` for text shaping, then rasterizes it to an image
/// which is then written to a `Texture`.
pub struct TextPipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    global: Rc<GlobalResources>,
    font_system: FontSystem,
    cache: SwashCache,
}

impl TextPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        global: Rc<GlobalResources>,
    ) -> Self {
        let mut font_system = FontSystem::new();
        font_system.db_mut().load_system_fonts();
        log::info!("Loaded system fonts");
        let cache = SwashCache::new();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into()),
        });

        let layout = BindGroupLayoutBuilder::new()
            .label("Text bind group layout")
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
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[global.window_layout(), &layout],
            push_constant_ranges: &[],
        });

        // TODO create a builder for this
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Render Pipeline"),
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
            font_system,
            pipeline,
            layout,
            global,
            cache,
        }
    }

    pub fn text_size(&mut self, text: &Text) -> Size {
        let font_system = &mut self.font_system;

        let mut buffer = Buffer::new(
            font_system,
            Metrics::new(
                text.font_size as f32,
                text.font_size as f32 * text.line_height,
            ),
        );

        // TODO try to get the default font on each platform
        // TODO expose font weight and other items
        // Just get any sans-serif font
        let attrs = Attrs::new().family(cosmic_text::Family::SansSerif);

        buffer.set_text(
            font_system,
            &text.text,
            attrs,
            cosmic_text::Shaping::Advanced,
        );

        buffer.shape_until_scroll(font_system, false);
        let mut size = Size::default();
        for run in buffer.layout_runs() {
            size.width += size.width.max(run.line_w); // Get the max of all lines
            size.height += run.line_height;
        }

        // Add padding to prevent clipping
        size += 1.0;

        size
    }

    /// Uses `comsic-text` to rasterize the font into a an image, which
    /// will then be written to a `wgpu::Buffer`.
    fn rasterize_text(&mut self, text: &Text) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, Size) {
        // FIXME there's some artifacts appearing on the texture.
        let font_system = &mut self.font_system;
        let cache = &mut self.cache;

        let mut buffer = Buffer::new(
            font_system,
            Metrics::new(
                text.font_size as f32,
                text.font_size as f32 * text.line_height,
            ),
        );

        // TODO try to get the default font on each platform
        // TODO expose font weight and other items
        // Just get any sans-serif font
        let attrs = Attrs::new().family(cosmic_text::Family::SansSerif);

        buffer.set_text(
            font_system,
            &text.text,
            attrs,
            cosmic_text::Shaping::Advanced,
        );

        buffer.shape_until_scroll(font_system, false);
        let mut size = Size::default();
        for run in buffer.layout_runs() {
            size.width += size.width.max(run.line_w); // Get the max of all lines
            size.height += run.line_height;
        }

        // Add padding to prevent clipping
        size += 1.0; // TODO change to 1.0

        let [r, g, b, a] = text.color.to_rgba();

        // FIXME one of the sizes starts at 0 and one starts at 1 im not sure which
        let mut image = RgbaImage::new(size.width as u32, size.height as u32);
        buffer.set_size(font_system, Some(size.width), Some(size.height));

        buffer.draw(
            font_system,
            cache,
            cosmic_text::Color::rgba(r, g, b, a),
            |x, y, _, _, color| {
                let x = x as u32;
                let y = y as u32;
                if x < image.width() && y < image.height() {
                    let pixel = image.get_pixel_mut(x, y);
                    *pixel = Rgba([color.r(), color.b(), color.g(), color.a()])
                }
            },
        );

        (image, size)
    }

    pub fn draw(
        &mut self,
        text: &Text,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass,
    ) {
        let (text_img, size) = self.rasterize_text(&text);

        let vertices = Vertex::quad(size, text.position, text.color);

        let vertex_buffer = BufferBuilder::new()
            .label("Text vertex buffer")
            .vertex()
            .init(&vertices)
            .build(device);

        let texture = TextureBuilder::new()
            .label("Text texture")
            .size(size)
            .dimension(wgpu::TextureDimension::D2)
            .format(wgpu::TextureFormat::Rgba8UnormSrgb)
            .usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(device);

        let texture_view = texture.create_view(&Default::default());
        let sampler = device.create_sampler(&Default::default());

        let bind_group = BindGroupBuilder::new()
            .label("Text bind group")
            .texture_view(&texture_view)
            .sampler(&sampler)
            .build(&self.layout, device);

        let size = Extent3d {
            width: size.width as u32,
            height: size.height as u32,
            depth_or_array_layers: 1,
        };

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &text_img,
            wgpu::TexelCopyBufferLayout {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup;

    #[tokio::test]
    async fn font_rasterizing() {
        let (device, _) = setup().await;
        let global = Rc::new(GlobalResources::new(&device, Size::default()));
        let format = wgpu::TextureFormat::Rgba8Unorm;

        let mut pipeline = TextPipeline::new(&device, format, global);
        let text_queue = [
            Text::new("Hello world").line_height(20.0),
            Text::new("Hello world").font_size(255),
            Text::new("Hi mom!"),
            Text::new("Click me please! You might get a treat")
                .line_height(2.0)
                .font_size(24),
        ];

        for text in text_queue {
            let (_image, _size) = pipeline.rasterize_text(&text);
        }
    }
}
