use super::GlobalResources;
use crate::{
    builders::{
        BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder, TextureBuilder,
        VertexBufferLayoutBuilder,
    },
    primitives::Image,
    vertex::Vertex,
};
use helium_core::{color::TRANSPARENT, Size,Position};
use image::{ImageBuffer, Rgba};
use std::{rc::Rc, time::Instant};
use wgpu::Extent3d;

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    global: Rc<GlobalResources>,
    sampler: wgpu::Sampler,
	atlas:TextureAtlas,
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

        let sampler = device.create_sampler(&Default::default());
		let atlas = TextureAtlas::new(device);

        Self {
            pipeline,
            layout,
            global,
            sampler,
			atlas,
        }
    }

    pub fn draw(
        &mut self,
        image: &Image,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass,
    ) {
        let instant = Instant::now();
        let quad_size = image.size;
        let image_size = Size::new(image.data.width() as f32, image.data.height() as f32);

		let uv = self.atlas.get(image,queue);

        let vertices = Vertex::quad_with_uv(quad_size, image.position, TRANSPARENT,uv);

        // HERE
        let vertex_buffer = BufferBuilder::new()
            .label("Image vertex buffer")
            .vertex()
            .init(&vertices)
            .build(device);

        // HERE
		// TODO move this to and maybe buffer into constructor
        let texture_view = self.atlas.texture.create_view(&Default::default());

        // HERE
        let bind_group = BindGroupBuilder::new()
            .label("Image bind group")
            .texture_view(&texture_view)
            .sampler(&self.sampler)
            .build(&self.layout, device);

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, self.global.window_bind_group(), &[]);
        pass.set_bind_group(1, &bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.draw(0..vertices.len() as u32, 0..1);

        log::trace!("Image pipeline total: {:?}", instant.elapsed());
    }
}

struct TextureAtlas{
	/// The position of the next image
	offset:Position,
	size:Size,
	texture: wgpu::Texture,
	images: Vec<CachedImage>,
}

impl TextureAtlas {
	fn new(device:&wgpu::Device) -> Self{
		// TODO add atlas size param
		// TODO add max texture and image size option
		let size = Size::new(6000.0, 6000.0);
		let texture = TextureBuilder::new()
            .label("Texture Atlas")
            .size(size)
            .dimension(wgpu::TextureDimension::D2)
            .format(wgpu::TextureFormat::Rgba8UnormSrgb)
            .usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(device);

		Self { 
			size,
			offset:Position::default(),
			texture,
			images:vec![]
		}
	}

	pub fn get(&mut self,image:&Image,queue:&wgpu::Queue) -> [[f32;2];4]{
		// Check if image already exists
		for cached_image in &self.images{
			if cached_image.image == image.data{
				return cached_image.uv(self.size)
			}
		}
		
        let size = Extent3d {
			width: image.data.width(),
            height: image.data.height(),
            depth_or_array_layers: 1,
        };

		let origin = wgpu::Origin3d{
			x:self.offset.x as u32,
			y:self.offset.y as u32,
			z:0,
		};
		
		let cached_image = CachedImage::new(self.offset, image.data.clone());
		let uv = cached_image.uv(self.size);
		self.images.push(cached_image);

		// TODO check for verical offset
		self.offset.x += size.width as f32;
		
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin,
                aspect: wgpu::TextureAspect::All,
            },
            &image.data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width as u32),
                rows_per_image: Some(size.height as u32),
            },
            size,
        );

		uv
	}
}

/// An image from the [`TextureAtlas`]
#[derive(PartialEq,Clone)]
struct CachedImage{
	/// The position of the image in the [`TextureAtlas`]
	location:Position,
	image:ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl CachedImage{
	fn new(location:Position,image:ImageBuffer<Rgba<u8>,Vec<u8>>) -> Self{
		Self{
			location,
			image
		}
	}

	/// Get the uv coordinates for the image
	fn uv(&self,atlas_size:Size) -> [[f32;2];4]{
		let size = Size::new(self.image.width() as f32, self.image.height() as f32);

		let top_left = [ 
			self.location.x / atlas_size.width,
			self.location.y / atlas_size.height,
		];

		let top_right = [ 
			(self.location.x + size.width) / atlas_size.width,
			self.location.y / atlas_size.height,
		];

		let bottom_right = [ 
			(self.location.x + size.width) / atlas_size.width,
			(self.location.y + size.height) / atlas_size.height,
		];

		let bottom_left = [ 
			self.location.x  / atlas_size.width,
			(self.location.y + size.height) / atlas_size.height,
		];

		/// uv's are defined clockwise
		let uv = [top_left,top_right,bottom_right,bottom_left];

		uv
	}
}

#[cfg(test)]
mod test{

}