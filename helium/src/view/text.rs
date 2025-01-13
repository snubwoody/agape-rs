use super::View;
use crate::{app::AppState, geometry::Vertex, resources::ResourceManager, Color, Position, Size};
use helium_core::color::BLACK;
use std::{collections::HashMap, fmt::Debug, io::Cursor};

#[derive(Debug, Clone, PartialEq)]
pub struct TextView {
    id: String,
    text: String,
    font_size: u8,
    color: Color,
    vertices: Vec<Vertex>,
    resources: HashMap<String, usize>,
}

impl TextView {
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            text: text.to_string(),
            font_size: 16,
            color: BLACK,
            vertices: vec![],
            resources: HashMap::new(),
        }
    }

    /// Set the `font_size` of the [`TextView`]
    pub fn font_size(mut self, font_size: u8) -> Self {
        self.font_size = font_size;
        self
    }

    fn to_vertices(&self, position: Position, size: Size) -> Vec<Vertex> {
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
                self.color.into_hex_string().as_str(),
            )
            .unwrap(); // TODO Hangle the errors pls

        let image = image::load(Cursor::new(text_image.data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();

        let texture = resources.add_texture("Text Texture", size, &state.device);

        let texture_view = resources.add_texture_view(texture)?;
        let vertex_buffer = resources.add_vertex_buffer_init(
            "Text Vertex Buffer",
            bytemuck::cast_slice(&vertices),
            &state.device,
        );

        let sampler = resources.add_sampler("Texture Sampler", &state.device);
        let bind_group = resources.add_bind_group(
            "Text Bind Group",
            &state.context.text_pipeline.texture_bind_group_layout,
            &state.device,
            &[],
            &[texture_view],
            &[sampler],
        )?;

        resources.write_texture(texture, size, &image, &state.queue)?;

        self.resources.insert("Texture".to_string(), texture);
        self.resources.insert("Bind group".to_string(), bind_group);
        self.resources
            .insert("Vertex buffer".to_string(), vertex_buffer);
        self.vertices = vertices;

        Ok(())
    }

	fn resize(
		&mut self, 
		layout: &dyn crystal::Layout, 
		resources: &ResourceManager, 
		state: &AppState
	) -> Result<(),crate::Error> {
		let position = layout.position();
		let size = layout.size();

		self.vertices = Vertex::quad(size, position, self.color);
		let vertex_buffer = self.resources.get("Vertex buffer").unwrap();

		resources.write_buffer(
			*vertex_buffer, 
			0, 
			bytemuck::cast_slice(&self.vertices), 
			&state.queue
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
        let vertex_buffer = resources
            .buffer(*self.resources.get("Vertex buffer").unwrap())
            .unwrap();
        let bind_group = resources
            .bind_group(*self.resources.get("Bind group").unwrap())
            .unwrap();

        pass.set_pipeline(&context.text_pipeline.pipeline);
        pass.set_bind_group(0, &context.text_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, &bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..self.vertices.len() as u32, 0..1);
    }
}
