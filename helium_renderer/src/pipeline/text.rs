use helium_core::Size;
use std::{io::Cursor, rc::Rc, time::Instant};
use wgpu::Extent3d;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, SwashCache};
use image::{ImageBuffer, Rgba, RgbaImage};
use super::GlobalResources;
use crate::{
    builders::{
        BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder, TextureBuilder,
        VertexBufferLayoutBuilder,
    },
    primitives::Text,
    vertex::Vertex,
};

// TODO replace text_to_png
/// The pipeline that handles text rendering.
/// It uses `cosmic_text` for text shaping, then rasterizes it to an image
/// which is then written to a `Texture`.
pub struct TextPipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    global: Rc<GlobalResources>,
	font_system:FontSystem,
	cache:SwashCache
}

impl TextPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        global: Rc<GlobalResources>,
    ) -> Self {
		let mut font_system = FontSystem::new();
		font_system.db_mut().load_system_fonts();
		log::trace!("Loaded system fonts");
		let mut cache = SwashCache::new();
		
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
			font_system,
            pipeline,
            layout,
            global,
			cache
        }
    }

	/// Uses `comsic-text` to rasterize the font into a an image, which
	/// will then be written to a `wgpu::Buffer`.
	fn rasterize_text(&mut self) -> ImageBuffer<Rgba<u8>,Vec<u8>>{
		let instant = Instant::now();
		let font_system = &mut self.font_system;
		let cache = &mut self.cache;
		
		let mut buffer = Buffer::new(font_system, Metrics::new(16.0, 16.0*1.5));
		
		// TODO try to get the default font on each platform
		// Just get any sans-serif font
		let attrs = Attrs::new().family(cosmic_text::Family::SansSerif);
		buffer.set_text(font_system, "Please sign-in", attrs, cosmic_text::Shaping::Advanced);
		
		let mut image = RgbaImage::new(200, 200);
		
		buffer.shape_until_scroll(font_system, false);
		buffer.draw(
			font_system, 
			cache, 
			cosmic_text::Color::rgb(0, 0, 0), 
			|x,y,_,_,color|{
				let pixel = image.get_pixel_mut(x as u32, y as u32);
				*pixel = Rgba([color.r(),color.b(),color.g(),color.a()])
			}
		);
		
		log::trace!("Rendered text in: {:?}",instant.elapsed());
		image
	}

    pub fn draw(
        &mut self,
        text: &Text,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass,
    ) {
        let text_renderer = text_to_png::TextRenderer::default();

        // Rasterize the text
        // Should hopefully replace this library eventually with something glyph based
        let text_image = text_renderer
            .render_text_to_png_data(
                text.text.clone(),
                text.font_size,
                text.color.into_hex_string().as_str(),
            )
            .unwrap();

        // FIXME return these errors
        let image = image::load(Cursor::new(text_image.data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();

        let size = Size {
            width: 200.0,
            height: 200.0,
        };
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
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.rasterize_text(),
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


#[cfg(test)]
mod tests {
    use ab_glyph::{point, Font};
    use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Scroll, SwashCache};
    use image::{RgbImage, Rgba, RgbaImage};

	#[test]
	fn draw_text(){
		let mut font_system = FontSystem::new();
		font_system.db_mut().load_system_fonts();

		let mut buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 16.0*1.5));

		let attrs = Attrs::new().family(cosmic_text::Family::SansSerif);
		buffer.set_text(&mut font_system, "Please sign-in", attrs, cosmic_text::Shaping::Advanced);
		let mut cache = SwashCache::new();

		let mut image = RgbaImage::new(200, 200);

		buffer.shape_until_scroll(&mut font_system, false);
		buffer.draw(
			&mut font_system, 
			&mut cache, 
			cosmic_text::Color::rgb(0, 0, 0), 
			|x,y,_,_,color|{
				let pixel = image.get_pixel_mut(x as u32, y as u32);
				*pixel = Rgba([color.r(),color.b(),color.g(),color.a()])
			}
		);

		image.save("text.png").unwrap();
	}
}
