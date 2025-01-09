use crate::{
    app::AppState,
    geometry::{vertex::Vertex, RenderContext},
    impl_surface,
    surface::Surface,
    Bounds, Color, Position, Size,
};
use std::fmt::Debug;
use wgpu::util::DeviceExt;

/// Draws images to the screen
pub struct ImageSurface {
    id: String,
    position: Position,
    size: Size,
    img: image::DynamicImage,
    texture: Option<wgpu::Texture>,
    sampler: Option<wgpu::Sampler>,
    view: Option<wgpu::TextureView>,
    bind_group: Option<wgpu::BindGroup>,
}

impl ImageSurface {
    pub fn new(id: &str, img: image::DynamicImage) -> Self {
        Self {
            id: id.to_string(),
            position: Position::new(0.0, 0.0),
            size: Size::default(),
            img,
            texture: None,
            sampler: None,
            view: None,
            bind_group: None,
        }
    }

    fn to_vertices(&self) -> Vec<Vertex> {
        let width = self.size.width;
        let height = self.size.height;
        let color = Color::default().normalize();
        let x = self.position.x;
        let y = self.position.y;

        let vertex1 = Vertex::new_with_uv(x, y, color, [0.0, 0.0]); //Top left
        let vertex2 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); // Top right
        let vertex3 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); //Bottom left
        let vertex4 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); //Top right
        let vertex5 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); // Bottom left
        let vertex6 = Vertex::new_with_uv(x + width, y + height, color, [1.0, 1.0]); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }
}

impl Surface for ImageSurface {
    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
        // FIXME issue with fill sizing causing overflow
        // FIXME wgpu panics if size is 0
        let vertices = self.to_vertices();

        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Set the render pipeline and vertex buffer
        render_pass.set_pipeline(&context.image_pipeline.pipeline);
        render_pass.set_bind_group(0, &context.image_pipeline.window_bind_group, &[]);
        render_pass.set_bind_group(1, self.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        render_pass.draw(0..vertices.len() as u32, 0..1);
    }

    fn build(&mut self, state: &AppState, context: &RenderContext) {
        // TODO maybe move this to the pipeline
        let texture_size = wgpu::Extent3d {
            width: self.size.width as u32,
            height: self.size.height as u32,
            depth_or_array_layers: 1,
        };

        let texture = state.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Image Texture"),
            view_formats: &[],
        });

        let texture_view = texture.create_view(&Default::default());
        let texture_sampler = state.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Image Texture sampler"),
            ..Default::default()
        });

        let texture_bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Image Texture bind group"),
            layout: &context.image_pipeline.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
            ],
        });

        self.texture = Some(texture);
        self.view = Some(texture_view);
        self.sampler = Some(texture_sampler);
        self.bind_group = Some(texture_bind_group);

        let img = self
            .img
            .resize(
                self.size.width as u32,
                self.size.height as u32,
                image::imageops::FilterType::Nearest, // This is by far the fastest filter type
            )
            .to_rgba8();

        state.queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: self.texture.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.size.width as u32), // TODO don't even know what this is
                rows_per_image: None,
            },
            texture_size,
        );
    }

    impl_surface!();
}

impl Debug for ImageSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageSurface")
            .field("size", &self.size)
            .field("position", &self.position)
            .finish()
    }
}
