use crate::{
    app::AppState,
    error::Error,
    geometry::{vertex::Vertex, RenderContext},
    resources::ResourceManager,
    view::View,
    Color, Position, Size,
};
use crystal::Layout;
use image::GenericImageView;
use std::{collections::HashMap, fmt::Debug};
use wgpu::util::DeviceExt;

#[derive(Debug, Clone, PartialEq)]
pub struct ImageView {
    id: String,
    image: ::image::DynamicImage,
    /// A map of all the resources needed by this view
    resources: HashMap<String, usize>,
}

impl ImageView {
    pub fn new(id: &str, image: ::image::DynamicImage) -> Self {
        Self {
            id: id.to_string(),
            image,
            resources: HashMap::new(),
        }
    }
}

impl View for ImageView {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(
        &mut self,
        layout: &dyn Layout,
        resources: &mut ResourceManager,
        state: &AppState,
    ) -> Result<(), Error> {
        let size = layout.size();
        let position = layout.position();

        let texture = resources.add_texture(
            "Image texture",
            size, // Textures cannot have dimensions of 0
            &state.device,
        );

        let view = resources.add_texture_view(texture)?;
        let sampler = resources.add_sampler("Image texture sampler", &state.device);

        let texture_size = wgpu::Extent3d {
            width: size.width as u32,
            height: size.height as u32,
            depth_or_array_layers: 1,
        };

        self.image.resize(
            size.width as u32,
            size.height as u32,
            image::imageops::FilterType::Nearest, // This is by far the fastest filter type
        );

        state.queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: resources.texture(texture).unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.image.to_rgba8(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width as u32), // TODO don't even know what this is
                rows_per_image: None,
            },
            texture_size,
        );

        let bind_group = resources.add_bind_group(
            "Image texture bind group",
            &state.context.image_pipeline.texture_bind_group_layout,
            &state.device,
            &[],
            &[view],
            &[sampler],
        )?;

        Ok(())
    }

    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
    }
}

/// Draws images to the screen
pub struct ImageSurface {
    id: String,
    position: Position,
    size: Size,
    img: image::DynamicImage,
    /// The index of the texture in the [`ResourceManager`].
    texture: usize,
    /// The index of the sampler in the [`ResourceManager`].
    sampler: usize,
    /// The index of the texture view in the [`ResourceManager`].
    view: usize,
    /// The index of the bind group in the [`ResourceManager`].
    bind_group: usize,
}

impl ImageSurface {
    pub fn new(
        id: &str,
        img: image::DynamicImage,
        context: &RenderContext,
        resources: &mut ResourceManager,
        device: &wgpu::Device,
    ) -> Result<Self, Error> {
        let texture = resources.add_texture(
            "Image texture",
            img.dimensions().into(), // Textures cannot have dimensions of 0
            device,
        );
        let view = resources.add_texture_view(texture)?;
        let sampler = resources.add_sampler("Image texture sampler", device);

        let bind_group = resources.add_bind_group(
            "Image texture bind group",
            &context.image_pipeline.texture_bind_group_layout,
            device,
            &[],
            &[view], // TODO not sure about this
            &[sampler],
        )?;

        Ok(Self {
            id: id.to_string(),
            position: Position::new(0.0, 0.0),
            size: Size::default(),
            img,
            texture,
            sampler,
            view,
            bind_group,
        })
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

impl ImageSurface {
    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
        let vertices = self.to_vertices();

        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Set the render pipeline and vertex buffer
        pass.set_pipeline(&context.image_pipeline.pipeline);
        pass.set_bind_group(0, &context.image_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, resources.bind_group(self.bind_group).unwrap(), &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..vertices.len() as u32, 0..1);
    }

    fn build(&mut self, state: &AppState, resources: &ResourceManager) {
        let texture_size = wgpu::Extent3d {
            width: self.size.width as u32,
            height: self.size.height as u32,
            depth_or_array_layers: 1,
        };
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
                texture: resources.texture(self.texture).unwrap(),
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
}

impl Debug for ImageSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageSurface")
            .field("size", &self.size)
            .field("position", &self.position)
            .finish()
    }
}
