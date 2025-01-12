use super::View;
use crate::{
    app::AppState, geometry::vertex::Vertex, resources::ResourceManager, widgets::icon::feather_icons::rss, Color, Position, Size
};
use helium_core::color::BLACK;
use image::RgbaImage;
use std::{collections::HashMap, fmt::Debug, io::Cursor};
use wgpu::util::DeviceExt;

#[derive(Debug, Clone, PartialEq, )]
pub struct TextView {
    id: String,
    text: String,
    font_size: u8,
    color: Color,
	vertices:Vec<Vertex>,
	resources:HashMap<String,usize>
}

impl TextView {
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            text: text.to_string(),
            font_size: 16,
            color: BLACK,
			vertices: vec![],
			resources:HashMap::new()
        }
    }

    /// Set the `font_size` of the [`TextView`]
    pub fn font_size(mut self, font_size: u8) -> Self {
        self.font_size = font_size;
        self
    }

	fn to_vertices(&self, position:Position,size:Size) -> Vec<Vertex> {
        let color = Color::default().normalize();
		let width = size.width;
		let height = size.height;
        let x = position.x;
        let y = position.y;

        let vertex1 = Vertex::new_with_uv(x, y, color, [0.0, 0.0]); //Top left
        let vertex2 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); // Top right
        let vertex3 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); //Bottom left
        let vertex4 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); //Top right
        let vertex5 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); // Bottom left
        let vertex6 = Vertex::new_with_uv(x + width, y + height, color, [1.0, 1.0]); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }

    /// Set the `font_size` of the [`TextView`]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for TextView {
    fn id(&self) -> &str {
        &self.id
    }

	/// Initialises the [`TextView`] to prepare it for rendering.
	/// 
	/// Uses the `text_to_png` to rasterize the text into a PNG image,
	/// which is then written to a texture.
    fn init(
        &mut self,
        layout: &dyn crystal::Layout,
        resources: &mut ResourceManager,
        state: &AppState,
    ) -> Result<(), crate::Error> {
		let position = layout.position();
		let size = layout.size();
		let vertices = self.to_vertices(position, size);

		let text_renderer = text_to_png::TextRenderer::default();
		
		// Render the text as a png
        let text_image = text_renderer
            .render_text_to_png_data(
				self.text.clone(), 
				self.font_size, 
				self.color.into_hex_string().as_str()
			)
            .unwrap(); // TODO Hangle the errors pls

        let image = image::load(Cursor::new(text_image.data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();

		let texture = resources.add_texture(
			"Text Texture", 
			size, 
			&state.device
		);

		let texture_view = resources.add_texture_view(texture)?;
		let vertex_buffer = resources.add_vertex_buffer_init(
			"Text Vertex Buffer", 
			bytemuck::cast_slice(&vertices), 
			&state.device
		);

		let sampler = resources.add_sampler("Texture Sampler", &state.device);
		let bind_group = resources.add_bind_group(
			"Text Bind Group", 
			&state.context.text_pipeline.texture_bind_group_layout, 
			&state.device, 
			&[], 
			&[texture_view], 
			&[sampler]
		)?;

		resources.write_texture(
			texture, 
			size, 
			&image, 
			&state.queue
		)?;

        self.resources.insert("Texture".to_string(), texture);
        self.resources.insert("Bind group".to_string(), bind_group);
        self.resources.insert("Vertex buffer".to_string(), vertex_buffer);
		self.vertices = vertices;

        Ok(())
    }

    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {

		let vertex_buffer = resources.buffer(
			*self.resources.get("Vertex buffer").unwrap()
		).unwrap();
		let bind_group = resources.bind_group(
			*self.resources.get("Bind group").unwrap()
		).unwrap();

        pass.set_pipeline(&context.text_pipeline.pipeline);
        pass.set_bind_group(0, &context.text_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, &bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..self.vertices.len() as u32, 0..1);

    }
}

#[derive(Clone)]
pub struct TextSurface {
    id: String,
    position: Position,
    size: Size,
    text: String,
    font_size: u8,
    color: Color,
    img: RgbaImage,
}

impl TextSurface {
    pub fn new(id: &str, text: &str, font_size: u8, color: &Color) -> Self {
        let text_renderer = text_to_png::TextRenderer::default();

        // Render the text as a png
        let text_image = text_renderer
            .render_text_to_png_data(text, font_size, color.into_hex_string().as_str())
            .unwrap(); // TODO Hangle the errors pls

        let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();

        Self {
            id: id.to_string(),
            position: Position::new(0.0, 0.0),
            size: Size::new(text_image.size.width as f32, text_image.size.height as f32),
            text: String::from(text),
            font_size,
            color: BLACK,
            img,
        }
    }

    /// Rasterize the text and return the texture
    pub fn prepare(&self, device: &wgpu::Device) -> (wgpu::Texture, wgpu::Extent3d) {
        let texture_size = wgpu::Extent3d {
            width: self.size.width as u32,
            height: self.size.height as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Text Texture"),
            view_formats: &[],
        });

        return (texture, texture_size);
    }

    fn to_vertices(&self, width: f32, height: f32) -> Vec<Vertex> {
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

impl TextSurface {
    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
        let (texture, texture_size) = self.prepare(&state.device);

        let vertices = self.to_vertices(texture_size.width as f32, texture_size.height as f32);

        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let texture_view = texture.create_view(&Default::default());
        let texture_sampler = state.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text bind group"),
            layout: &context.text_pipeline.texture_bind_group_layout,
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

        state.queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.size.width as u32),
                rows_per_image: Some(self.size.height as u32),
            },
            texture_size,
        );

        // Set the render pipeline and vertex buffer
        pass.set_pipeline(&context.text_pipeline.pipeline);
        pass.set_bind_group(0, &context.text_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, &texture_bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..vertices.len() as u32, 0..1);
    }
}

impl Debug for TextSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextSurface")
            .field("size", &self.size)
            .field("position", &self.position)
            .field("text", &self.text)
            .field("font_size", &self.font_size)
            .field("color", &self.color)
            .finish()
    }
}
